// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is dual-licensed under either the MIT license found in the
// LICENSE-MIT file in the root directory of this source tree or the Apache
// License, Version 2.0 found in the LICENSE-APACHE file in the root directory
// of this source tree. You may select, at your option, one of the above-listed licenses.

//! A trait representing a Path OSAM stash.

use crate::{
    bucket::{Bucket, PathOsamBlock},
    utils::{bitonic_sort_by_keys, CompleteBinaryTreeIndex, TreeIndex},
    BucketSize, Identifier, OsamBlock, OsamError, StashSize,
};
use std::collections::HashSet;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};

const STASH_GROWTH_INCREMENT: usize = 10;

#[derive(Debug)]
/// A fixed-size, obliviously accessed Path OSAM stash data structure implemented using oblivious sorting.
pub struct ObliviousStash<V: OsamBlock> {
    blocks: Vec<PathOsamBlock<V>>,
    path_size: StashSize,
    evict_path_size: StashSize,
}

impl<V: OsamBlock> ObliviousStash<V> {
    pub fn len(&self) -> usize {
        self.blocks.len()
    }
}

impl<V: OsamBlock> ObliviousStash<V> {
    pub fn new<const Z: BucketSize>(
        path_size: StashSize,
        overflow_size: StashSize,
    ) -> Result<Self, OsamError> {
        // Create a stash of `path_size + evict_path_size + overflow_size`. The first `path_size + evict_path_size`
        // indices are used for downloading and uploading data from the server.
        let evict_path_size = path_size - StashSize::try_from(Z)?;
        let num_stash_blocks: usize = (path_size + evict_path_size + overflow_size).try_into()?;

        Ok(Self {
            blocks: vec![PathOsamBlock::<V>::dummy(); num_stash_blocks],
            path_size,
            evict_path_size,
        })
    }

    // Write server-side path from root to leaf.
    pub fn write_to_path<const Z: BucketSize>(
        &mut self,
        physical_memory: &mut [Bucket<V, Z>],
        position: TreeIndex,
    ) -> Result<(), OsamError> {
        let height = position.ct_depth();
        let mut level_assignments = vec![TreeIndex::MAX; self.len()];
        let mut level_counts = vec![0; usize::try_from(height)? + 1];

        // Assign all non-dummy blocks in the stash to either the path or the overflow.
        for (i, block) in self.blocks.iter().enumerate() {
            // If `block` is a dummy, the rest of this loop iteration will be a no-op, and the values don't matter.
            let block_is_dummy = block.ct_is_dummy();

            // Set up valid but meaningless input to the computation in case `block` is a dummy.
            let an_arbitrary_leaf: TreeIndex = 1 << height;
            let block_position =
                TreeIndex::conditional_select(&block.position, &an_arbitrary_leaf, block_is_dummy);

            // Assign the block to a bucket or to the overflow.
            let mut assigned = Choice::from(0);
            // Obliviously scan through the buckets from leaf to root,
            // assigning the block to the first empty bucket satisfying the invariant.
            for (level, count) in level_counts.iter_mut().enumerate().rev() {
                let level_bucket_full: Choice = count.ct_eq(&(u64::try_from(Z)?));

                let level_u64 = u64::try_from(level)?;
                let level_satisfies_invariant = block_position
                    .ct_node_on_path(level_u64, height)
                    .ct_eq(&position.ct_node_on_path(level_u64, height));

                let should_assign = level_satisfies_invariant
                    & (!level_bucket_full)
                    & (!block_is_dummy)
                    & (!assigned);
                assigned |= should_assign;

                let level_count_incremented = *count + 1;
                count.conditional_assign(&level_count_incremented, should_assign);
                level_assignments[i].conditional_assign(&level_u64, should_assign);
            }
            // If the block was not able to be assigned to any bucket, assign it to the overflow.
            level_assignments[i]
                .conditional_assign(&(TreeIndex::MAX - 1), (!assigned) & (!block_is_dummy));
        }

        // Assign dummy blocks to the remaining non-full buckets until all buckets are full.
        let mut exists_unfilled_levels: Choice = 1.into();
        let mut first_unassigned_block_index: usize = 0;
        // Need to pad `evict_path_size` slots with dummy blocks so real
        // blocks are not overwritten by future path downloads.
        let mut reserve_to_fill = self.evict_path_size;
        // Unless the stash overflows, this loop will execute exactly once, and the inner `if` will not execute.
        // If the stash overflows, this loop will execute twice and the inner `if` will execute.
        // This difference in control flow will leak the fact that the stash has overflowed.
        // This is a violation of obliviousness, but the alternative is simply to fail.
        // If the stash is set large enough when the OSAM is initialized,
        // stash overflow will occur only with negligible probability.
        while exists_unfilled_levels.into() {
            // Make a pass over the stash, assigning dummy blocks to unfilled levels in the path.
            for (i, block) in self
                .blocks
                .iter()
                .enumerate()
                .skip(first_unassigned_block_index)
            {
                let block_free = block.ct_is_dummy();

                // Assign to buckets that are not full.
                let mut assigned: Choice = 0.into();
                for (level, count) in level_counts.iter_mut().enumerate() {
                    let full = count.ct_eq(&(u64::try_from(Z)?));
                    let no_op = assigned | full | !block_free;

                    level_assignments[i].conditional_assign(&(u64::try_from(level))?, !no_op);
                    count.conditional_assign(&(*count + 1), !no_op);
                    assigned |= !no_op;
                }

                // Real blocks that are assigned to the overflow have the assignment `TreeIndex::Max-1` so
                // they appear at the start of the stash / end of `reserve_space` before any dummy blocks.
                // Assign dummy blocks to `TreeIndex::Max - 2` to pad out `reserve_space` so they appear
                // before any real blocks that were assigned to the overflow.
                let open_reserve_space = reserve_to_fill.ct_ne(&0);
                let reserve_to_fill_decremented = reserve_to_fill.saturating_sub(1);
                let assign_to_reserve = (!assigned) & open_reserve_space & block_free;
                level_assignments[i].conditional_assign(&(TreeIndex::MAX - 2), assign_to_reserve);
                reserve_to_fill.conditional_assign(&reserve_to_fill_decremented, assign_to_reserve);
            }

            // Check that all levels have been filled.
            exists_unfilled_levels = reserve_to_fill.ct_ne(&0);
            for count in level_counts.iter() {
                let full = count.ct_eq(&(u64::try_from(Z)?));
                exists_unfilled_levels |= !full;
            }

            // If not, there must not have been enough dummy blocks remaining in the stash.
            // That is, the stash has overflowed.
            // So, extend the stash with STASH_GROWTH_INCREMENT more dummy blocks,
            // and repeat the process of trying to fill all unfilled levels with dummy blocks.
            if exists_unfilled_levels.into() {
                first_unassigned_block_index = self.blocks.len();

                self.blocks.resize(
                    self.blocks.len() + STASH_GROWTH_INCREMENT,
                    PathOsamBlock::<V>::dummy(),
                );
                level_assignments.resize(
                    level_assignments.len() + STASH_GROWTH_INCREMENT,
                    TreeIndex::MAX,
                );

                log::warn!(
                    "Stash overflow occurred. Stash resized to {} blocks.",
                    self.blocks.len()
                );
            }
        }

        // Sort stash so the first `path_size` blocks align with their assigned buckets.
        bitonic_sort_by_keys(&mut self.blocks, &mut level_assignments);

        // Write the first Z * height blocks into slots in the tree.
        for depth in 0..=height {
            let bucket_to_write =
                &mut physical_memory[usize::try_from(position.ct_node_on_path(depth, height))?];
            for slot_number in 0..Z {
                let stash_index = (usize::try_from(depth)?) * Z + slot_number;
                bucket_to_write.blocks[slot_number] = self.blocks[stash_index];
                self.blocks[stash_index] = PathOsamBlock::<V>::dummy();
            }
        }

        Ok(())
    }

    /// Read a server-side path from root to leaf into the first `path_size` indices.
    /// Then, read another path  into the next `evict_path_size` indices.
    /// Any blocks that are overlapping on both paths are downloaded only once.
    pub fn read_from_path<const Z: BucketSize>(
        &mut self,
        physical_memory: &mut [Bucket<V, Z>],
        position: TreeIndex,
        evict_position: TreeIndex,
    ) -> Result<(), OsamError> {
        let height = position.ct_depth();

        // Download physical memory to stash and replace with dummy blocks.
        let mut checked_buckets = HashSet::new();
        for i in 0..(self.path_size / u64::try_from(Z)?) {
            let bucket_index = usize::try_from(position.ct_node_on_path(i, height))?;
            checked_buckets.insert(bucket_index);
            let mut bucket = physical_memory[bucket_index];
            for slot_index in 0..Z {
                self.blocks[Z * (usize::try_from(i)?) + slot_index] = bucket.blocks[slot_index];
                bucket.blocks[slot_index] = PathOsamBlock::<V>::dummy();
            }
            physical_memory[bucket_index] = bucket;
        }

        // Download physical memory to stash from evict path, skipping any buckets that were already collected.
        let offset = usize::try_from(self.path_size)?;
        for i in 1..(self.path_size / u64::try_from(Z)?) {
            let bucket_index = usize::try_from(evict_position.ct_node_on_path(i, height))?;
            if !checked_buckets.contains(&bucket_index) {
                let mut bucket = physical_memory[bucket_index];
                for slot_index in 0..Z {
                    self.blocks[offset + Z * (usize::try_from(i)? - 1) + slot_index] =
                        bucket.blocks[slot_index];
                    bucket.blocks[slot_index] = PathOsamBlock::<V>::dummy();
                }
                physical_memory[bucket_index] = bucket;
            }
        }

        Ok(())
    }

    /// Write block to stash by overwriting the leftmost dummy block.
    pub fn write_to_stash(
        &mut self,
        identifier: Identifier,
        position: TreeIndex,
        value: V,
    ) -> Result<(), OsamError> {
        // Create block with new values.
        let new_block = PathOsamBlock {
            value,
            identifier,
            position,
        };

        // Overwrite the first dummy block.
        let mut assigned = Choice::from(0);
        let buffer: usize = usize::try_from(self.path_size + self.evict_path_size)?;

        // Skip the first `path_size` indices in the stash, since these are
        // overwritten upon calling `read_from_path`.
        for block in self.blocks.iter_mut().skip(buffer) {
            let block_is_dummy = block.ct_is_dummy();
            let should_assign = block_is_dummy & (!assigned);
            assigned |= should_assign;
            block.conditional_assign(&new_block, should_assign);
        }

        // Add block to stash and resize if there no room currently (stash overflow).
        if (!assigned).into() {
            self.blocks.push(new_block);

            self.blocks.resize(
                self.blocks.len() + STASH_GROWTH_INCREMENT,
                PathOsamBlock::<V>::dummy(),
            );

            log::warn!(
                "Stash overflow occurred. Stash resized to {} blocks.",
                self.blocks.len()
            );
        }

        Ok(())
    }

    /// Read block from stash and replace with dummy.
    pub fn read_from_stash(&mut self, identifier: Identifier) -> Result<Option<V>, OsamError> {
        let mut result: V = V::default();
        let mut found: Choice = 0.into();

        // Iterate over stash, updating the block with identifier `identifier` if one exists.
        for block in &mut self.blocks {
            let is_requested_index = block.identifier.ct_eq(&identifier);
            found.conditional_assign(&1.into(), is_requested_index);

            // Read current value of target block into `result`.
            result.conditional_assign(&block.value, is_requested_index);
            // Write new position into target block.
            block.conditional_assign(&PathOsamBlock::<V>::dummy(), is_requested_index);
        }

        // Return the value of the found block or None.
        let mut output: Option<V> = None;
        if found.into() {
            output = Some(result);
        }

        // Return the value of the found block (or the default value, if no block was found).
        Ok(output)
    }

    /// Outputs the number of real blocks in the stash.
    pub fn occupancy(&self) -> StashSize {
        let mut result = 0;
        for i in self.path_size.try_into().unwrap()..(self.blocks.len()) {
            if (!self.blocks[i].ct_is_dummy()).into() {
                result += 1;
            }
        }
        result
    }

    /// Print blocks in stash for debug purposes.
    pub fn print_stash(&self) {
        print!("STASH: ");
        for i in (self.path_size + self.evict_path_size).try_into().unwrap()..self.blocks.len() {
            let block = self.blocks[i];
            if (!block.ct_is_dummy()).into() {
                print!(
                    "({}, {}, {:?}) ",
                    block.identifier, block.position, block.value
                );
            }
        }
        println!();
    }
}
