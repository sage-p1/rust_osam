// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is dual-licensed under either the MIT license found in the
// LICENSE-MIT file in the root directory of this source tree or the Apache
// License, Version 2.0 found in the LICENSE-APACHE file in the root directory
// of this source tree. You may select, at your option, one of the above-listed licenses.

//! Block and bucket structures for Path OSAM.

use crate::{BlockSize, OsamBlock};
use subtle::{Choice, ConditionallySelectable};

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::BucketSize;

use crate::{Identifier, utils::TreeIndex};
use subtle::ConstantTimeEq;

#[derive(Clone, Copy, Debug, PartialEq)]
/// An `OsamBlock` consisting of unstructured bytes.
pub struct BlockValue<const B: BlockSize> {
    /// The block's data payload.
    pub data: [u8; B],
}
impl<const B: BlockSize> BlockValue<B> {
    /// Instantiates a `BlockValue` from an array of `BLOCK_SIZE` bytes.
    pub fn new(data: [u8; B]) -> Self {
        Self { data }
    }
}

impl<const B: BlockSize> Default for BlockValue<B> {
    fn default() -> Self {
        BlockValue::<B> { data: [0u8; B] }
    }
}

impl<const B: BlockSize> OsamBlock for BlockValue<B> {}

impl<const B: BlockSize> ConditionallySelectable for BlockValue<B> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        let mut result = BlockValue::default();
        for i in 0..B {
            result.data[i] = u8::conditional_select(&a.data[i], &b.data[i], choice);
        }
        result
    }
}

impl<const B: BlockSize> Distribution<BlockValue<B>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BlockValue<B> {
        let mut result = BlockValue::default();
        for i in 0..B {
            result.data[i] = rng.gen();
        }
        result
    }
}

#[derive(Clone, Copy, Default, PartialEq)]
/// A Path OSAM block combines an `OsamBlock` V with two metadata fields; its OSAM `identifier` and its `position` in the tree.
pub(crate) struct PathOsamBlock<V> {
    pub value: V,
    pub identifier: Identifier,
    pub position: TreeIndex,
}

impl<V: OsamBlock> PathOsamBlock<V> {
    const DUMMY_IDENTIFIER: Identifier = Identifier::MAX;
    const DUMMY_POSITION: TreeIndex = 0;

    pub fn dummy() -> Self {
        Self {
            value: V::default(),
            identifier: Self::DUMMY_IDENTIFIER,
            position: Self::DUMMY_POSITION,
        }
    }

    pub fn ct_is_dummy(&self) -> Choice {
        self.position.ct_eq(&Self::DUMMY_POSITION)
    }

    #[cfg(test)]
    pub fn is_dummy(&self) -> bool {
        self.position == Self::DUMMY_POSITION
    }
}

impl<V: OsamBlock> std::fmt::Debug for PathOsamBlock<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.ct_is_dummy().into() {
            write!(f, "PathOsamBlock::Dummy")
        } else {
            f.debug_struct("PathOsamBlock")
                .field("value", &self.value)
                .field("identifier", &self.identifier)
                .field("position", &self.position)
                .finish()
        }
    }
}

impl<V: OsamBlock> OsamBlock for PathOsamBlock<V> {}

impl<V: ConditionallySelectable> ConditionallySelectable for PathOsamBlock<V> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        let value = V::conditional_select(&a.value, &b.value, choice);
        let identifier = Identifier::conditional_select(&a.identifier, &b.identifier, choice);
        let position = TreeIndex::conditional_select(&a.position, &b.position, choice);
        PathOsamBlock::<V> {
            value,
            identifier,
            position,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
/// A Path OSAM bucket.
pub struct Bucket<V: OsamBlock, const Z: BucketSize> {
    /// The Path OSAM blocks stored by this bucket.
    pub(crate) blocks: [PathOsamBlock<V>; Z],
}

impl<V: OsamBlock, const Z: BucketSize> std::fmt::Debug for Bucket<V, Z> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut self_is_dummy = true;

        for block in self.blocks {
            if (!block.ct_is_dummy()).into() {
                self_is_dummy = false;
            }
        }

        if self_is_dummy {
            write!(f, "Bucket::Dummy")
        } else {
            f.debug_struct("Bucket")
                .field("blocks", &self.blocks)
                .finish()
        }
    }
}

impl<V: OsamBlock, const Z: BucketSize> Default for Bucket<V, Z> {
    fn default() -> Self {
        Self {
            blocks: [PathOsamBlock::<V>::dummy(); Z],
        }
    }
}

impl<V: OsamBlock, const Z: BucketSize> ConditionallySelectable for Bucket<V, Z> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        let mut result = Self::default();
        for i in 0..result.blocks.len() {
            result.blocks[i] =
                PathOsamBlock::<V>::conditional_select(&a.blocks[i], &b.blocks[i], choice)
        }
        result
    }
}

impl<V: OsamBlock, const Z: BucketSize> OsamBlock for Bucket<V, Z> {}
