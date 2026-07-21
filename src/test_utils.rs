// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is dual-licensed under either the MIT license found in the
// LICENSE-MIT file in the root directory of this source tree or the Apache
// License, Version 2.0 found in the LICENSE-APACHE file in the root directory
// of this source tree. You may select, at your option, one of the above-listed licenses.

//! This module contains common test utilities for crates generating tests utilizing the
//! `osam` crate.

use std::collections::HashMap;
use std::sync::Once;
static INIT: Once = Once::new();
use crate::path_osam::PathOsam;
use crate::{BucketSize, Identifier, Osam, OsamBlock, OsamError, StashSize, TreeIndex};
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

/// Tests the correctness of OSAM on a sequence of all writes then reads.
pub(crate) fn write_then_read<T: Osam>(osam: &mut T, num_operations: usize, probability: f64)
where
    Standard: Distribution<T::V>,
{
    init_logger();
    let mut rng = StdRng::seed_from_u64(0);
    let mut mirror_hash_map = HashMap::new();

    // Generate a sequence of allocs and write a random value.
    for _ in 0..num_operations {
        let address = osam.alloc(&mut rng).unwrap();
        let identifier = address.0;
        let position = address.1;
        let random_block_value = rng.gen::<T::V>();
        let ordered_evict = rng.gen_bool(probability);

        mirror_hash_map.insert(address, random_block_value);
        let _ = osam.write(
            identifier,
            position,
            random_block_value,
            ordered_evict,
            &mut rng,
        );
    }

    // Assert reads fetch the proper data block.
    for (address, random_block_value) in mirror_hash_map.iter() {
        let identifier = address.0;
        let position = address.1;
        let ordered_evict = rng.gen_bool(probability);

        assert_eq!(
            osam.read(identifier, position, ordered_evict, &mut rng)
                .unwrap()
                .unwrap(),
            *random_block_value
        );
    }
}

/// Tests the correctness of Path OSAM on a sequence of all reads then writes.
pub(crate) fn read_then_write<T: Osam>(osam: &mut T, num_operations: usize, probability: f64)
where
    Standard: Distribution<T::V>,
{
    init_logger();
    let mut rng = StdRng::seed_from_u64(0);

    // Generate a sequence of allocs to read and then write a random value.
    for _ in 0..num_operations {
        let address = osam.alloc(&mut rng).unwrap();
        let identifier = address.0;
        let position = address.1;
        let ordered_evict = rng.gen_bool(probability);
        assert_eq!(
            osam.read(identifier, position, ordered_evict, &mut rng)
                .unwrap(),
            None
        );

        let ordered_evict = rng.gen_bool(probability);
        let _ = osam.write(
            identifier,
            position,
            T::V::default(),
            ordered_evict,
            &mut rng,
        );
    }
}

/// Tests the correctness of Path OSAM on a sequence where:
/// 1) the first half of writes are made to the OSAM.
/// 2) read half of these writes (quarter of all writes).
/// 3) the second half of writes are done.
/// 4) read all remaining writes.
pub(crate) fn interspersed_write_and_read<T: Osam>(
    osam: &mut T,
    num_operations: usize,
    probability: f64,
) where
    Standard: Distribution<T::V>,
{
    init_logger();
    let mut rng = StdRng::seed_from_u64(0);
    let mut mirror_hash_map = HashMap::new();

    let half = num_operations.checked_div(2).unwrap();
    let quarter = num_operations.checked_div(4).unwrap();

    // Generate the first half of allocs and write a random value.
    for _ in 0..half {
        let address = osam.alloc(&mut rng).unwrap();
        let identifier = address.0;
        let position = address.1;
        let random_block_value = rng.gen::<T::V>();
        let ordered_evict = rng.gen_bool(probability);

        mirror_hash_map.insert(address, random_block_value);
        let _ = osam.write(
            identifier,
            position,
            random_block_value,
            ordered_evict,
            &mut rng,
        );
    }

    // Assert reads fetch the proper data block for half the first writes (quarter of all).
    let mut used_addresses = Vec::new();
    let mut counter = 0;
    for (address, random_block_value) in mirror_hash_map.iter() {
        if counter >= quarter {
            break;
        }
        let identifier = address.0;
        let position = address.1;
        let ordered_evict = rng.gen_bool(probability);

        assert_eq!(
            osam.read(identifier, position, ordered_evict, &mut rng)
                .unwrap()
                .unwrap(),
            *random_block_value
        );
        used_addresses.push(address.to_owned());
        counter += 1;
    }

    // Remove used addresses to avoid double reading.
    for address in used_addresses.iter() {
        mirror_hash_map.remove(address);
    }

    // Generate the second half of allocs and write a random value.
    for _ in half..num_operations {
        let address = osam.alloc(&mut rng).unwrap();
        let identifier = address.0;
        let position = address.1;
        let random_block_value = rng.gen::<T::V>();
        let ordered_evict = rng.gen_bool(probability);

        mirror_hash_map.insert(address, random_block_value);
        let _ = osam.write(
            identifier,
            position,
            random_block_value,
            ordered_evict,
            &mut rng,
        );
    }

    // Assert the remaining three quarters of reads are correct.
    for (address, random_block_value) in mirror_hash_map.iter() {
        let identifier = address.0;
        let position = address.1;
        let ordered_evict = rng.gen_bool(probability);
        assert_eq!(
            osam.read(identifier, position, ordered_evict, &mut rng)
                .unwrap()
                .unwrap(),
            *random_block_value
        );
    }
}

/// Tests the correctness of PathOsam on a sequence of all writes then reads.
pub(crate) fn local_write_then_read<V: OsamBlock, const Z: BucketSize>(
    osam: &mut PathOsam<V, Z>,
    num_operations: usize,
    probability: f64,
) where
    Standard: Distribution<V>,
{
    init_logger();
    let mut rng = StdRng::seed_from_u64(0);
    let mut mirror_hash_map = HashMap::new();

    // Generate a sequence of allocs and write a random value.
    for _ in 0..num_operations {
        let address = osam.alloc(&mut rng).unwrap();
        let identifier = address.0;
        let position = address.1;
        let random_block_value = rng.gen::<V>();

        mirror_hash_map.insert(address, random_block_value);
        let _ = osam.local_write(identifier, position, random_block_value);
    }

    // Assert reads fetch the proper data block.
    for (address, random_block_value) in mirror_hash_map.iter() {
        let identifier = address.0;
        let position = address.1;
        let ordered_evict = rng.gen_bool(probability);

        assert_eq!(
            osam.read(identifier, position, ordered_evict, &mut rng)
                .unwrap()
                .unwrap(),
            *random_block_value
        );
    }
}

/// Tests the correctness of Path OSAM on a sequence where:
/// 1) the first quarter of writes are locally to the stash.
/// 2) read all of these writes.
/// 3) the last three quarters writes are made to the OSAM.
/// 4) read all remaining writes.
pub(crate) fn locally_interspersed_write_and_read<V: OsamBlock, const Z: BucketSize>(
    osam: &mut PathOsam<V, Z>,
    num_operations: usize,
    probability: f64,
) where
    Standard: Distribution<V>,
{
    init_logger();
    let mut rng = StdRng::seed_from_u64(0);
    let mut mirror_hash_map = HashMap::new();
    let quarter = num_operations.checked_div(4).unwrap();

    // Generate the first half of allocs and write a random value.
    for _ in 0..quarter {
        let address = osam.alloc(&mut rng).unwrap();
        let identifier = address.0;
        let position = address.1;
        let random_block_value = rng.gen::<V>();

        mirror_hash_map.insert(address, random_block_value);
        let _ = osam.local_write(identifier, position, random_block_value);
    }

    // Assert reads fetch the proper data block for half the first writes (quarter of all).
    for (address, random_block_value) in mirror_hash_map.iter() {
        let identifier = address.0;
        let position = address.1;
        let ordered_evict = rng.gen_bool(probability);

        assert_eq!(
            osam.read(identifier, position, ordered_evict, &mut rng)
                .unwrap()
                .unwrap(),
            *random_block_value
        );
    }
    mirror_hash_map.clear();

    // Generate the second half of allocs and write a random value.
    for _ in quarter..num_operations {
        let address = osam.alloc(&mut rng).unwrap();
        let identifier = address.0;
        let position = address.1;
        let random_block_value = rng.gen::<V>();
        let ordered_evict = rng.gen_bool(probability);

        mirror_hash_map.insert(address, random_block_value);
        let _ = osam.write(
            identifier,
            position,
            random_block_value,
            ordered_evict,
            &mut rng,
        );
    }

    // Assert the remaining three quarters of reads are correct.
    for (address, random_block_value) in mirror_hash_map.iter() {
        let identifier = address.0;
        let position = address.1;
        let ordered_evict = rng.gen_bool(probability);

        assert_eq!(
            osam.read(identifier, position, ordered_evict, &mut rng)
                .unwrap()
                .unwrap(),
            *random_block_value
        );
    }
}

// Runs all OSAM correctness tests.
// Uses a probability of 0.5 to toggle between deterministic and random eviction.
macro_rules! create_path_osam_correctness_tests_all_parameters {
    ($prefix: literal, $block_capacity: expr, $block_size: expr, $bucket_size: expr, $overflow_size: expr, $operation_factor: expr) => {
        paste::paste! {
            #[test]
            fn [<"write_then_read" $prefix $block_capacity _ $block_size _ $bucket_size _ $overflow_size _ $operation_factor>]() {
                let mut osam = PathOsam::<BlockValue<$block_size>, $bucket_size>::new_with_parameters($block_capacity, $overflow_size).unwrap();
                let num_operations = osam.block_capacity() * $bucket_size * $operation_factor + usize::try_from($overflow_size).unwrap().checked_div(2).unwrap();
                write_then_read(&mut osam, num_operations, 0.5);
            }

            #[test]
            fn [<"read_then_write" $prefix $block_capacity _ $block_size _ $bucket_size _ $overflow_size _ $operation_factor>]() {
                let mut osam = PathOsam::<BlockValue<$block_size>, $bucket_size>::new_with_parameters($block_capacity, $overflow_size).unwrap();
                let num_operations = osam.block_capacity() * $bucket_size * $operation_factor + usize::try_from($overflow_size).unwrap().checked_div(2).unwrap();
                read_then_write(&mut osam, num_operations, 0.5);
            }

            #[test]
            fn [<"interspersed_write_and_read" $prefix $block_capacity _ $block_size _ $bucket_size _ $overflow_size _ $operation_factor>]() {
                let mut osam = PathOsam::<BlockValue<$block_size>, $bucket_size>::new_with_parameters($block_capacity, $overflow_size).unwrap();
                let num_operations = osam.block_capacity() * $bucket_size * $operation_factor + usize::try_from($overflow_size).unwrap().checked_div(2).unwrap();
                interspersed_write_and_read(&mut osam, num_operations, 0.5);
            }

            #[test]
            fn [<"local_write_then_read" $prefix $block_capacity _ $block_size _ $bucket_size _ $overflow_size _ $operation_factor>]() {
                let mut osam = PathOsam::<BlockValue<$block_size>, $bucket_size>::new_with_parameters($block_capacity, $overflow_size).unwrap();
                let num_operations = osam.block_capacity() * $bucket_size * $operation_factor + usize::try_from($overflow_size).unwrap().checked_div(2).unwrap();
                local_write_then_read(&mut osam, num_operations, 0.5);
            }

            #[test]
            fn [<"locally_interspersed_write_and_read" $prefix $block_capacity _ $block_size _ $bucket_size _ $overflow_size _ $operation_factor>]() {
                let mut osam = PathOsam::<BlockValue<$block_size>, $bucket_size>::new_with_parameters($block_capacity, $overflow_size).unwrap();
                let num_operations = osam.block_capacity() * $bucket_size * $operation_factor + usize::try_from($overflow_size).unwrap().checked_div(2).unwrap();
                locally_interspersed_write_and_read(&mut osam, num_operations, 0.5);
            }
        }
    };
}

// Runs OSAM correctness tests relevant to small stash size.
// Uses a probability of 1.0 to always use deterministic eviction,
// which allows for maintaining a smaller stash.
macro_rules! create_path_osam_stash_size_correctness_tests_all_parameters {
    ($prefix: literal, $block_capacity: expr, $block_size: expr, $bucket_size: expr, $overflow_size: expr) => {
        paste::paste! {
            #[test]
            fn [<"write_then_read" $prefix $block_capacity _ $block_size _ $bucket_size _ $overflow_size>]() {
                let mut osam = StashSizeMonitor::<BlockValue<$block_size>, $bucket_size>::new_with_parameters($block_capacity, $overflow_size).unwrap();
                let num_operations = (osam.block_capacity() * $bucket_size).checked_div(2).unwrap();
                write_then_read(&mut osam, num_operations, 1.0);
            }

            #[test]
            fn [<"read_then_write" $prefix $block_capacity _ $block_size _ $bucket_size _ $overflow_size>]() {
                let mut osam = StashSizeMonitor::<BlockValue<$block_size>, $bucket_size>::new_with_parameters($block_capacity, $overflow_size).unwrap();
                let num_operations = (osam.block_capacity() * $bucket_size).checked_div(2).unwrap();
                read_then_write(&mut osam, num_operations, 1.0);
            }

            #[test]
            fn [<"interspersed_write_and_read" $prefix $block_capacity _ $block_size _ $bucket_size _ $overflow_size>]() {
                let mut osam = StashSizeMonitor::<BlockValue<$block_size>, $bucket_size>::new_with_parameters($block_capacity, $overflow_size).unwrap();
                let num_operations = (osam.block_capacity() * $bucket_size).checked_div(2).unwrap();
                interspersed_write_and_read(&mut osam, num_operations, 1.0);
            }
        }
    };
}

macro_rules! create_path_osam_correctness_tests_helper {
    ($prefix: literal, $bucket_size: expr, $overflow_size: expr) => {
        create_path_osam_correctness_tests_all_parameters!(
            $prefix,
            8,
            1,
            $bucket_size,
            $overflow_size,
            1
        );
        create_path_osam_correctness_tests_all_parameters!(
            $prefix,
            4,
            1,
            $bucket_size,
            $overflow_size,
            1
        );
        create_path_osam_correctness_tests_all_parameters!(
            $prefix,
            4,
            2,
            $bucket_size,
            $overflow_size,
            2
        );
        create_path_osam_correctness_tests_all_parameters!(
            $prefix,
            16,
            1,
            $bucket_size,
            $overflow_size,
            3
        );
        create_path_osam_correctness_tests_all_parameters!(
            $prefix,
            2,
            1,
            $bucket_size,
            $overflow_size,
            1
        );
    };
}

macro_rules! create_path_osam_stash_size_correctness_tests_helper {
    ($prefix: literal, $bucket_size: expr, $overflow_size: expr) => {
        create_path_osam_stash_size_correctness_tests_all_parameters!(
            $prefix,
            8,
            1,
            $bucket_size,
            $overflow_size
        );
        create_path_osam_stash_size_correctness_tests_all_parameters!(
            $prefix,
            4,
            1,
            $bucket_size,
            $overflow_size
        );
        create_path_osam_stash_size_correctness_tests_all_parameters!(
            $prefix,
            4,
            2,
            $bucket_size,
            $overflow_size
        );
        create_path_osam_stash_size_correctness_tests_all_parameters!(
            $prefix,
            16,
            1,
            $bucket_size,
            $overflow_size
        );
        create_path_osam_stash_size_correctness_tests_all_parameters!(
            $prefix,
            2,
            1,
            $bucket_size,
            $overflow_size
        );
    };
}

macro_rules! create_path_osam_correctness_tests {
    ($bucket_size: expr, $overflow_size: expr) => {
        create_path_osam_correctness_tests_helper!("_", $bucket_size, $overflow_size);
    };
}

macro_rules! create_path_osam_stash_size_correctness_tests {
    ($bucket_size: expr, $overflow_size: expr) => {
        create_path_osam_stash_size_correctness_tests_helper!(
            "_stash_size_",
            $bucket_size,
            $overflow_size
        );
    };
}

// Interface that shares Osam trait to ensure the stash does not overflow with small enough parameters.
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
            osam: PathOsam::new_with_parameters(block_capacity, overflow_size).unwrap(),
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
        Ok(self.osam.alloc(rng)?)
    }

    fn write<R: Rng + CryptoRng>(
        &mut self,
        identifier: Identifier,
        position: TreeIndex,
        value: V,
        ordered_evict: bool,
        rng: &mut R,
    ) -> Result<(), OsamError> {
        let _ = self
            .osam
            .write(identifier, position, value, ordered_evict, rng)?;
        let stash_size = self.osam.stash_occupancy();
        assert!(stash_size < 10);
        Ok(())
    }

    fn read<R: Rng + CryptoRng>(
        &mut self,
        identifier: Identifier,
        position: TreeIndex,
        ordered_evict: bool,
        rng: &mut R,
    ) -> Result<Option<V>, OsamError> {
        let result = self.osam.read(identifier, position, ordered_evict, rng)?;
        let stash_size = self.osam.stash_occupancy();
        assert!(stash_size < 10);
        Ok(result)
    }
}

pub(crate) use create_path_osam_correctness_tests;
pub(crate) use create_path_osam_correctness_tests_all_parameters;
pub(crate) use create_path_osam_correctness_tests_helper;
pub(crate) use create_path_osam_stash_size_correctness_tests;
pub(crate) use create_path_osam_stash_size_correctness_tests_all_parameters;
pub(crate) use create_path_osam_stash_size_correctness_tests_helper;
