#![allow(unused_variables)] // TODO(you): remove this lint after implementing this mod
#![allow(dead_code)] // TODO(you): remove this lint after implementing this mod

use bytes::BufMut;
use crate::block::{SZ_U16, SZ_U8};
use super::Block;

/// Builds a block.
pub struct BlockBuilder {
    block_size: usize,
    data_blocks: Vec<u8>,
    offsets: Vec<u16>,
}

impl BlockBuilder {
    /// Creates a new block builder.
    pub fn new(block_size: usize) -> Self {
        BlockBuilder {
            block_size,
            data_blocks: Vec::new(),
            offsets: Vec::new(),
        }
    }

    fn estimated_size(&self) -> usize {
        (self.data_blocks.len() * SZ_U8) + (self.offsets.len() * SZ_U16) + SZ_U16
    }

    /// Adds a key-value pair to the block. Returns false when the block is full.
    #[must_use]
    pub fn add(&mut self, key: &[u8], value: &[u8]) -> bool {
        let current_size = self.estimated_size();
        let k_size = key.len() * SZ_U8;
        let v_size = value.len() * SZ_U8;
        let insertion_overhead = k_size + v_size + 3 * SZ_U16;
        if insertion_overhead + current_size > self.block_size && !self.is_empty() {
            return false;
        }

        self.offsets.push(self.data_blocks.len() as u16);
        self.data_blocks.put_u16_le(key.len() as u16);
        self.data_blocks.put(key);
        self.data_blocks.put_u16_le(value.len() as u16);
        self.data_blocks.put(value);

        true
    }

    /// Check if there is no key-value pair in the block.
    pub fn is_empty(&self) -> bool {
        self.estimated_size() == 2
    }

    /// Finalize the block.
    pub fn build(self) -> Block {
        Block {
            data: self.data_blocks,
            offsets: self.offsets,
        }
    }
}
