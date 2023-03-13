use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::sync::Arc;
use crate::block::SZ_U16;

use super::Block;

/// Iterates on a block.
pub struct BlockIterator {
    block: Arc<Block>,
    idx: usize,
}

impl BlockIterator {
    fn new(block: Arc<Block>) -> Self {
        Self {
            block,
            idx: 0,
        }
    }

    /// Creates a block iterator and seek to the first entry.
    pub fn create_and_seek_to_first(block: Arc<Block>) -> Self {
        BlockIterator::new(block)
    }

    /// Creates a block iterator and seek to the first key that >= `key`.
    pub fn create_and_seek_to_key(block: Arc<Block>, key: &[u8]) -> Self {
        let mut block_iter = BlockIterator::new(block);
        block_iter.seek_to_key(key);
        block_iter
    }

    /// Returns the key of the current entry.
    pub fn key(&self) -> &[u8] {
        let offset = self.block.offsets[self.idx] as usize;
        let key_len = u16::from_le_bytes([self.block.data[offset], self.block.data[offset + 1]]) as usize;
        &self.block.data[offset + SZ_U16..offset + SZ_U16 + key_len]
    }

    /// Returns the value of the current entry.
    pub fn value(&self) -> &[u8] {
        let offset = self.block.offsets[self.idx] as usize + SZ_U16 + self.key().len();
        let val_len = u16::from_le_bytes([self.block.data[offset], self.block.data[offset + 1]]) as usize;
        &self.block.data[offset + SZ_U16..offset + SZ_U16 + val_len]
    }

    /// Returns true if the iterator is valid.
    pub fn is_valid(&self) -> bool {
        self.idx < self.block.offsets.len()
    }

    /// Seeks to the first key in the block.
    pub fn seek_to_first(&mut self) {
        self.idx = 0
    }

    /// Move to the next key in the block.
    pub fn next(&mut self) {
        self.idx += 1
    }

    /// Seek to the first key that >= `key`.
    pub fn seek_to_key(&mut self, key: &[u8]) {
        // TODO change with binary search

        while self.is_valid() {
            match self.key().cmp(key) {
                Ordering::Less => self.next(),
                Ordering::Equal => break,
                Ordering::Greater => break
            }
        }
        if !self.is_valid() {
            self.seek_to_first();
        }
    }
}
