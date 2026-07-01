// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is dual-licensed under either the MIT license found in the
// LICENSE-MIT file in the root directory of this source tree or the Apache
// License, Version 2.0 found in the LICENSE-APACHE file in the root directory
// of this source tree. You may select, at your option, one of the above-listed licenses.

//! An implementation of Path OSAM.

use super::stash::ObliviousStash;
use crate::{
    bucket::Bucket,
    utils::{CompleteBinaryTreeIndex, TreeHeight, TreeIndex},
    Identifier, BucketSize, Osam, OsamBlock, OsamError, StashSize,
};
use rand::{CryptoRng, Rng};
use bit_reverse::ParallelReverse;

/// The parameter "Z" from the Path OSAM literature that sets the number of blocks per bucket; typical values are 3 or 4.
/// Here we adopt the more conservative setting of 4.
pub const DEFAULT_BLOCKS_PER_BUCKET: BucketSize = 4;

/// The default number of overflow blocks that the Path OSAM stash (and recursive stashes) can store.
pub const DEFAULT_STASH_OVERFLOW_SIZE: StashSize = 40;

/// A doubly oblivious Path OSAM.
///
/// ## Parameters
///
/// - Block type `V`: the type of elements stored by the OSAM.
/// - Bucket size `Z`: the number of blocks per Path OSAM bucket.
///   Must be at least 2. Typical values are 3, 4, or 5.
///   Along with the overflow size, this value affects the probability
///   of stash overflow (see below) and should be set with care.
/// - Overflow size: The number of blocks that the stash can store between OSAM accesses without overflowing.
///   Along with the bucket size, this value affects the probability of stash overflow (see below)
///   and should be set with care.
///
/// ## Security
///
/// OSAM operations are guaranteed to be oblivious, *unless* the stash overflows.
/// In this case, the stash will grow, which reveals that the overflow occurred.
/// This is a violation of obliviousness, but a mild one in several ways.
/// The stash overflow is very likely to reset to empty after the overflow,
/// and stash overflows are isolated events. It is not at all obvious
/// how an attacker might use a stash overflow to infer properties of the access pattern.
///
/// That said, it is best to choose parameters so that the stash does not ever overflow.
/// With Z = 4, experiments from the [original Path OSAM paper](https://eprint.iacr.org/2013/280.pdf)
/// indicate that the probability of overflow is independent of the number N of blocks stored,
/// and that setting SO = 40 is enough to reduce this probability to below 2^{-50} (Figure 3).
/// The authors conservatively estimate that setting SO = 89 suffices for 2^{-80} overflow probability.
/// The choice Z = 3 is also popular, although the probability of overflow is less well understood.
#[derive(Debug)]
pub struct PathOsam<V: OsamBlock, const Z: BucketSize> {
    /// The underlying untrusted memory that the OSAM is obliviously accessing on behalf of its client.
    physical_memory: Vec<Bucket<V, Z>>,
    /// The Path OSAM stash.
    stash: ObliviousStash<V>,
    /// The height of the Path OSAM tree data structure.
    height: TreeHeight,
    /// The counter that assigns identifiers to Path OSAM blocks.
    identifier_counter: Identifier,
    /// The counter that deterministically picks which path evict.
    evict_counter: Identifier,
}

impl<V: OsamBlock, const Z: BucketSize> PathOsam<V, Z> {
    /// Returns a new `PathOsam` of default `V` values
    /// with a stash overflow size of `overflow_size` blocks
    /// (See [`PathOsam`]) for a description of these parameters).
    ///
    /// # Errors
    ///
    /// Returns an `InvalidConfigurationError` in the following cases.
    ///
    /// - `block_capacity` is 0, 1, or is not a power of two.
    /// - `Z` is 0 or 1.
    /// - `overflow_size` is 0.
    pub fn new_with_parameters(
        block_capacity: Identifier,
        overflow_size: StashSize,
    ) -> Result<Self, OsamError> {
        log::info!("PathOsam::new(capacity = {})", block_capacity,);

        if !block_capacity.is_power_of_two() | (block_capacity <= 1) {
            return Err(OsamError::InvalidConfigurationError {
                parameter_name: "OSAM capacity".to_string(),
                parameter_value: block_capacity.to_string(),
            });
        }

        if Z <= 1 {
            return Err(OsamError::InvalidConfigurationError {
                parameter_name: "Bucket size Z".to_string(),
                parameter_value: Z.to_string(),
            });
        }

        if overflow_size == 0 {
            return Err(OsamError::InvalidConfigurationError {
                parameter_name: "Overflow size".to_string(),
                parameter_value: overflow_size.to_string(),
            });
        }

        let number_of_nodes = block_capacity;
        let height: u64 = (block_capacity.ilog2() - 1).into();
        let path_size = u64::try_from(Z)? * (height + 1);
        let stash = ObliviousStash::new(path_size, overflow_size)?;

        // physical_memory holds `block_capacity` buckets, each storing up to Z blocks.
        // The number of leaves is `block_capacity` / 2, which the original Path OSAM paper's experiments
        // found was sufficient to keep the stash size small with high probability.
        let mut physical_memory = Vec::new();
        physical_memory.resize(usize::try_from(number_of_nodes)?, Bucket::<V, Z>::default());

        let identifier_counter: u64 = 1;
        let evict_counter: u64 = 0;

        Ok(Self {
            physical_memory,
            stash,
            height,
            identifier_counter,
            evict_counter,
        })
    }

    #[cfg(test)]
    pub(crate) fn stash_occupancy(&self) -> StashSize {
        self.stash.occupancy()
    }
}

impl<V: OsamBlock, const Z: BucketSize> Osam for PathOsam<V, Z> {
    type V = V;

    fn block_capacity(&self) -> usize {
        self.physical_memory.len()
    }

    fn alloc<R: Rng + CryptoRng>(
        &mut self,
        rng: &mut R,
    ) -> Result<(Identifier, TreeIndex), OsamError> {
        // Assign unique identifier from counter
        let identifier = self.identifier_counter;
        self.identifier_counter += 1;

        // Randomly select leaf position 
        let position = CompleteBinaryTreeIndex::random_leaf(self.height, rng)?;
        Ok((identifier, position))
    }

    fn write<R: Rng + CryptoRng>(
        &mut self,
        new_identifier: Identifier,
        new_position: TreeIndex,
        new_value: Self::V,
        rng: &mut R,
    ) -> Result<(), OsamError> {
        assert_ne!(new_identifier, Identifier::MAX);
        assert!(new_position.is_leaf(self.height));

        // Read a dummy path to make reads and writes indistinguishable
        let dummy_position: TreeIndex = CompleteBinaryTreeIndex::random_leaf(self.height, rng)?;
        assert!(dummy_position.is_leaf(self.height));
        self.stash.read_from_path(&mut self.physical_memory, dummy_position)?;

        // Add new block to stash by replacing a dummy block
        self.stash.write_to_stash(new_identifier, new_position, new_value)?;

        // Evict blocks from the stash into the path that was just read,
        // replacing them with dummy blocks.
        let evict_position = self.evict_position()?;
        self.stash.write_to_path(&mut self.physical_memory, evict_position)?;

        Ok(())
    }

    fn read(
        &mut self,
        identifier: Identifier,
        position: TreeIndex,
    ) -> Result<Option<Self::V>, OsamError> {
        assert_ne!(identifier, Identifier::MAX);
        assert!(position.is_leaf(self.height));

        // Read path containing target block
        self.stash.read_from_path(&mut self.physical_memory, position)?;

        // Remove block from stash (and replace with dummy)
        let result = self.stash.read_from_stash(identifier)?;

        // Evict blocks from the stash into the path that was just read,
        // replacing them with dummy blocks.
        let evict_position = self.evict_position()?;
        self.stash.write_to_path(&mut self.physical_memory, evict_position)?;

        Ok(result)
    }

    fn evict_position(&mut self) -> Result<TreeIndex, OsamError> {
        // Deterministically evict buckets in reverse-lexicographic ordering
        let mut evict_position: TreeIndex = self.evict_counter;
        let height: u32 = self.height.try_into()?;
        let num_leaves = 2u64.pow(height);
        evict_position %= num_leaves; // Map to bucket indices
        evict_position = evict_position.swap_bits(); // Bit reversal
        evict_position = evict_position.checked_shr(64 - height).unwrap_or_else(|| 0); // Move bits over to leaf indices
        evict_position += num_leaves; // Add bucket offset
        self.evict_counter += 1;
        Ok(evict_position)
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{bucket::*, test_utils::*};

    // Test default parameters. For the small capacity used in the tests, this means a linear position map.
    create_path_osam_correctness_tests!(4, 40);

    // Test small initial stash sizes and correct resizing of stash on overflow.
    create_path_osam_correctness_tests!(4, 10);
    create_path_osam_correctness_tests!(4, 1);

    // Test small and large bucket sizes.
    create_path_osam_correctness_tests!(3, 40);
    create_path_osam_correctness_tests!(5, 40);

    // Check that the stash size stays reasonably small over the test runs.
    create_path_osam_stash_size_tests!(4, 40);
}
