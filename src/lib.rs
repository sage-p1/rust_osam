// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is dual-licensed under either the MIT license found in the
// LICENSE-MIT file in the root directory of this source tree or the Apache
// License, Version 2.0 found in the LICENSE-APACHE file in the root directory
// of this source tree. You may select, at your option, one of the above-listed licenses.

//! An implementation of Oblivious SAM (OSAM) for the secure enclave setting.
//!
//! ⚠️ **Warning**: This implementation has not been audited. Use at your own risk!
//!
//! # Overview
//!
//! This crate implements an oblivious SAM protocol (OSAM) for (secure) enclave applications.
//!
//! This crate assumes that OSAM clients are running inside a secure enclave architecture that provides memory encryption.
//! It does not perform encryption-on-write and thus is **not** secure without memory encryption.
//!
//! # Design
//!
//! This crate implements the Path OSAM protocol, with oblivious
//! client data structures based on the [Oblix paper](https://people.eecs.berkeley.edu/~raluca/oblix.pdf).
//! See the [Path OSAM retrospective paper](http://elaineshi.com/docs/pathosam-retro.pdf)
//! for a high-level introduction to OSAM and Path OSAM, and for more detailed references.
//!
//! # Example
//!
//! The below example reads a database from memory into an OSAM, thus permitting secret-dependent accesses.
//!
//! ```
//! use osam::{Identifier, BlockSize, BlockValue, Osam, DefaultOsam};
//! # use osam::OsamError;
//!
//! const BLOCK_SIZE: BlockSize = 64;
//! const DB_SIZE: Identifier = 64;
//! # const DATABASE: [[u8; BLOCK_SIZE as usize]; DB_SIZE as usize] =
//! # [[0; BLOCK_SIZE as usize]; DB_SIZE as usize];
//! let mut rng = rand::rngs::OsRng;
//!
//! // Initialize an OSAM to store 64 blocks of 64 bytes each.
//! let mut osam = DefaultOsam::<BlockValue<BLOCK_SIZE>>::new(DB_SIZE, &mut rng)?;
//!
//! // Read a database (here, an array of byte arrays) into the OSAM.
//! for (i, bytes) in DATABASE.iter().enumerate() {
//!     osam.write(i as Address, BlockValue::new(*bytes), &mut rng)?;
//! }
//!
//! // Now you can safely make secret-dependent accesses to your database.
//! let secret = 42;
//! let _ = osam.read(secret, &mut rng)?;
//! # Ok::<(), OsamError>(())
//! ```
//!
//! # Advanced
//!
//! OSAMs can store arbitrary structs implementing `OsamBlock`.
//! We provide implementations of `OsamBlock` for `u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64`,
//! and `BlockValue<const B: BlockSize>`.
//!
//! The `DefaultOsam` used in the above example should have good performance in most use cases.
//! But the underlying algorithms have several tunable parameters that impact performance.
//! The following example instantiates the same OSAM struct as above, but using the `PathOsam`
//! interface which exposes these parameters.
//!
//! ```
//! use osam::{Address, BlockSize, BlockValue, BucketSize,
//!             Osam, PathOsam, StashSize};
//! use osam::path_osam::{DEFAULT_BLOCKS_PER_BUCKET, DEFAULT_RECURSION_CUTOFF,
//!             DEFAULT_POSITIONS_PER_BLOCK, DEFAULT_STASH_OVERFLOW_SIZE};
//! # use osam::OsamError;
//! # let mut rng = rand::rngs::OsRng;
//! # const BLOCK_SIZE: BlockSize = 64;
//! # const DB_SIZE: Address = 64;
//!
//! const BUCKET_SIZE: BucketSize = DEFAULT_BLOCKS_PER_BUCKET;
//! const POSITIONS_PER_BLOCK: BlockSize = DEFAULT_POSITIONS_PER_BLOCK;
//! const INITIAL_STASH_OVERFLOW_SIZE: StashSize = DEFAULT_STASH_OVERFLOW_SIZE;
//!
//! let mut osam = PathOsam::<
//!     BlockValue<BLOCK_SIZE>,
//!     BUCKET_SIZE,
//!     POSITIONS_PER_BLOCK,
//!     >::new_with_parameters(DB_SIZE, &mut rng, INITIAL_STASH_OVERFLOW_SIZE, RECURSION_CUTOFF)?;
//! # Ok::<(), OsamError>(())
//! ```
//!
//! See [`PathOsam`] for an explanation of these parameters and their possible settings.

#![warn(clippy::cargo, clippy::doc_markdown, missing_docs, rustdoc::all)]

use std::num::TryFromIntError;

use rand::{CryptoRng, RngCore};
use subtle::ConditionallySelectable;
use thiserror::Error;
use utils::TreeIndex;

pub(crate) mod bucket;
pub mod path_osam;
pub(crate) mod stash;
#[cfg(test)]
mod test_utils;
pub(crate) mod utils;

pub use crate::bucket::BlockValue;
pub use crate::path_osam::PathOsam;

/// The numeric type used to specify the size of an OSAM block in bytes.
pub type BlockSize = usize;
/// The numeric type used to assign a unique identifier to a block.
pub type Identifier = u64;
/// The numeric type used to specify the size of an OSAM bucket in blocks.
pub type BucketSize = usize;
/// Numeric type used to represent the size of a Path OSAM stash in blocks.
pub type StashSize = u64;

/// A "trait alias" for OSAM blocks: the values read and written by OSAMs.
pub trait OsamBlock:
    Copy + Clone + std::fmt::Debug + Default + PartialEq + ConditionallySelectable
{
}

impl OsamBlock for u8 {}
impl OsamBlock for u16 {}
impl OsamBlock for u32 {}
impl OsamBlock for u64 {}
impl OsamBlock for i8 {}
impl OsamBlock for i16 {}
impl OsamBlock for i32 {}
impl OsamBlock for i64 {}

/// A list of error types which are produced during OSAM protocol execution.
#[derive(Error, Debug)]
pub enum OsamError {
    /// Errors arising from conversions between integer types.
    #[error("Arithmetic error encountered.")]
    IntegerConversionError(#[from] TryFromIntError),
    /// Errors arising from invalid parameters or configuration.
    #[error("Invalid configuration. {parameter_name} cannot have value {parameter_value}.")]
    InvalidConfigurationError {
        /// The misconfigured parameter.
        parameter_name: String,
        /// Its invalid value.
        parameter_value: String,
    },
}

/// Represents an oblivious SAM (OSAM) mapping identifiers of type `Identifier` 
/// and positions of type `TreeIndex` to values of type `V: OsamBlock`.
pub trait Osam
where
    Self: Sized,
{
    /// The type of elements stored in the OSAM.
    type V: OsamBlock;

    /// Returns the capacity in blocks of this OSAM.
    fn block_capacity(&self) -> usize;

    /// Allocates a valid Identifier and TreeIndex
    fn alloc<R: RngCore + CryptoRng>(
        &mut self,
        rng: &mut R,
    ) -> Result<(Identifier, TreeIndex), OsamError>;

    /// Obliviously writes the value stored at `index`. Returns the value previously stored at `index`.
    fn write<R: RngCore + CryptoRng>(
        &mut self,
        identifier: Identifier,
        position: TreeIndex,
        new_value: Self::V,
        rng: &mut R,
    ) -> Result<(), OsamError>;

    /// Obliviously reads the value stored at `index`.
    fn read(
        &mut self,
        identifier: Identifier,
        position: TreeIndex,
    ) -> Result<Option<Self::V>, OsamError>;

    /// Calculates the next position to evict
    fn evict_position(&mut self) -> Result<TreeIndex, OsamError>;
}
