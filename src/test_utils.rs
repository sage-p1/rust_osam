// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is dual-licensed under either the MIT license found in the
// LICENSE-MIT file in the root directory of this source tree or the Apache
// License, Version 2.0 found in the LICENSE-APACHE file in the root directory
// of this source tree. You may select, at your option, one of the above-listed licenses.

//! This module contains common test utilities for crates generating tests utilizing the
//! `osam` crate.

use std::fmt::Debug;
use std::sync::Once;
use std::ops::Div;
use std::collections::HashMap;
static INIT: Once = Once::new();
use crate::path_osam::PathOsam;
use crate::{
    BucketSize, Identifier, Osam, OsamBlock, OsamError, StashSize, TreeIndex
};
use rand::{
    distributions::{Distribution, Standard},
    rngs::StdRng,
    CryptoRng, Rng, SeedableRng,
};
use simplelog::{Config, WriteLogger};

// For use in manual testing and inspection.
// Change log_level to "Warn" to see stash overflow events, and to "Debug" to additionally see OSAM initialization events.
pub(crate) fn init_logger() {
    INIT.call_once(|| {
        WriteLogger::init(
            log::LevelFilter::Error,
            Config::default(),
            std::io::stdout(),
        )
        .unwrap()
    })
}

/// Tests the correctness of an `OSAM` implementation T on a sequence of all writes then reads
pub(crate) fn write_and_evict_then_read<T: Osam>(osam: &mut T, bucket_size: usize, operation_factor: usize, stash_size_experiment: bool)
where
    Standard: Distribution<T::V>,
{
    init_logger();
    let mut rng = StdRng::seed_from_u64(0);

    // let num_operations = (capacity * bucket_size).div(2);
    let capacity = osam.block_capacity();
    let num_operations: usize;
    if stash_size_experiment {
        num_operations = (capacity * bucket_size).div(2);
    } else {
        num_operations = capacity * bucket_size * operation_factor;
    }
    
    let mut mirror_hash_map = HashMap::new();

    // Generate a sequence of allocs and write a random value
    for _ in 0..num_operations {
        let address = osam.alloc(&mut rng).unwrap();
        let identifier = address.0;
        let position = address.1;
        let random_block_value = rng.gen::<T::V>();
        let _ = osam.write_and_evict(identifier, position, random_block_value, &mut rng);
        mirror_hash_map.insert(address, random_block_value);
    }

    // Assert reads fetch the proper data block
    for (address, &random_block_value) in mirror_hash_map.iter() {
        let identifier = address.0;
        let position = address.1;
        assert_eq!(
            osam.read(identifier, position).unwrap().unwrap(),
            random_block_value
        )
    }
}

/// Tests the correctness of an `OSAM` implementation T on a sequence of all reads then writes
pub(crate) fn read_then_write_and_evict<T: Osam>(osam: &mut T, bucket_size: usize, operation_factor: usize, stash_size_experiment: bool)
where
    Standard: Distribution<T::V>,
{
    init_logger();
    let mut rng = StdRng::seed_from_u64(0);

    let capacity = osam.block_capacity();
    let num_operations: usize;
    if stash_size_experiment {
        num_operations = (capacity * bucket_size).div(2);
    } else {
        num_operations = capacity * bucket_size * operation_factor;
    }

    // Generate a sequence of allocs and write a random value
    for _ in 0..num_operations {
        let address = osam.alloc(&mut rng).unwrap();
        let identifier = address.0;
        let position = address.1;
        assert_eq!(
            osam.read(identifier, position).unwrap(),
            None
        );
        let _ = osam.write_and_evict(identifier, position, T::V::default(), &mut rng);
    }
}

macro_rules! create_path_osam_correctness_tests_all_parameters {
    ($osam_type: ident, $prefix: literal, $block_capacity: expr, $block_size: expr, $bucket_size: expr, $overflow_size: expr, $operation_factor: expr, $stash_size_experiment: expr) => {
        paste::paste! {
            #[test]
            fn [<"write_and_evict_then_read" $prefix $block_capacity _ $block_size _ $bucket_size _ $overflow_size _ $operation_factor _ $stash_size_experiment>]() {
                let mut osam = $osam_type::<BlockValue<$block_size>, $bucket_size>::new_with_parameters($block_capacity, $overflow_size).unwrap();
                write_and_evict_then_read(&mut osam, $bucket_size, $operation_factor, $stash_size_experiment);
            }

            #[test]
            fn [<"read_then_write_and_evict" $prefix $block_capacity _ $block_size _ $bucket_size _ $overflow_size _ $operation_factor _ $stash_size_experiment>]() {
                let mut osam = $osam_type::<BlockValue<$block_size>, $bucket_size>::new_with_parameters($block_capacity, $overflow_size).unwrap();
                read_then_write_and_evict(&mut osam, $bucket_size, $operation_factor, $stash_size_experiment);
            }
        }
    };
}

macro_rules! create_path_osam_correctness_tests_helper {
    ($osam_type: ident, $prefix: literal, $bucket_size: expr, $overflow_size: expr, $stash_size_experiment: expr) => {
        create_path_osam_correctness_tests_all_parameters!(
            $osam_type,
            $prefix,
            8,
            1,
            $bucket_size,
            $overflow_size,
            1,
            $stash_size_experiment
        );
        create_path_osam_correctness_tests_all_parameters!(
            $osam_type,
            $prefix,
            4,
            1,
            $bucket_size,
            $overflow_size,
            1,
            $stash_size_experiment
        );
        // Block size 4 blocks, block size 2 bytes, testing with 100 operations
        create_path_osam_correctness_tests_all_parameters!(
            $osam_type,
            $prefix,
            4,
            2,
            $bucket_size,
            $overflow_size,
            2,
            $stash_size_experiment
        );
        create_path_osam_correctness_tests_all_parameters!(
            $osam_type,
            $prefix,
            16,
            1,
            $bucket_size,
            $overflow_size,
            3,
            $stash_size_experiment
        );
        create_path_osam_correctness_tests_all_parameters!(
            $osam_type,
            $prefix,
            2,
            1,
            $bucket_size,
            $overflow_size,
            1,
            $stash_size_experiment
        );
    };
}

macro_rules! create_path_osam_correctness_tests {
    ($bucket_size: expr, $overflow_size: expr) => {
        create_path_osam_correctness_tests_helper!(
            PathOsam,
            "",
            $bucket_size,
            $overflow_size,
            false
        );
    };
}

macro_rules! create_path_osam_stash_size_tests {
    ($bucket_size: expr, $overflow_size: expr) => {
        create_path_osam_correctness_tests_helper!(
            StashSizeMonitor,
            "_stash_size_",
            $bucket_size,
            $overflow_size,
            true
        );
    };
}

#[derive(Debug)]
pub(crate) struct StashSizeMonitor<V: OsamBlock, const Z: BucketSize> {
    osam: PathOsam<V, Z>,
}

impl<V: OsamBlock, const Z: BucketSize> StashSizeMonitor<V, Z> {
    pub(crate) fn new_with_parameters(
        block_capacity: Identifier,
        overflow_size: StashSize,
    ) -> Result<Self, OsamError> {
        Ok(Self {
            osam: PathOsam::new_with_parameters(
                block_capacity,
                overflow_size,
            )
            .unwrap(),
        })
    }
}

impl<V: OsamBlock, const Z: BucketSize> Osam for StashSizeMonitor<V, Z> {
    type V = V;

    fn block_capacity(&self) -> usize {
        self.osam.block_capacity()
    }

    fn alloc<R: Rng + CryptoRng>(
        &mut self,
        rng: &mut R,
    ) -> Result<(Identifier, TreeIndex), OsamError> {
        self.osam.alloc(rng)
    }

    fn write_and_evict<R: Rng + CryptoRng>(
        &mut self,
        new_identifier: Identifier,
        new_position: TreeIndex,
        new_value: Self::V,
        rng: &mut R,
    ) -> Result<(), OsamError> {
        let result = self.osam.write_and_evict(new_identifier, new_position, new_value, rng);
        let stash_size = self.osam.stash_occupancy();
        assert!(stash_size < 10);
        result
    }

    fn write_no_evict(
        &mut self,
        new_identifier: Identifier,
        new_position: TreeIndex,
        new_value: Self::V,
    ) -> Result<(), OsamError> {
        let result = self.osam.write_no_evict(new_identifier, new_position, new_value);
        let stash_size = self.osam.stash_occupancy();
        assert!(stash_size < 10);
        result
    }

    fn read(
        &mut self,
        identifier: Identifier,
        position: TreeIndex,
    ) -> Result<Option<Self::V>, OsamError> {
        let result = self.osam.read(identifier, position);
        let stash_size = self.osam.stash_occupancy();
        assert!(stash_size < 10);
        result
    }

    fn evict_position(&mut self) -> Result<TreeIndex, OsamError> {
        self.osam.evict_position()
    }
}

pub(crate) use create_path_osam_correctness_tests;
pub(crate) use create_path_osam_correctness_tests_all_parameters;
pub(crate) use create_path_osam_correctness_tests_helper;
pub(crate) use create_path_osam_stash_size_tests;
