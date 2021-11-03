pub(crate) mod error;
pub(crate) mod iter;

use std::collections;
use crate::decode::error::DecoderCreationError;

pub struct SinglePermutationDecoded<'a, 'b> {
    s: &'a [u8],
    key: &'b [usize],
}

impl<'a, 'b> SinglePermutationDecoded<'a, 'b> {
    pub fn new(s: &'a [u8],  key: &'b [usize]) -> Result<Self, DecoderCreationError> {
        if key.len() == 0 { return Err(DecoderCreationError::EmptyKey) }

        let key_entries: collections::HashSet<usize> = key.iter().map(|r| *r).collect();
        let q: collections::HashSet<usize> = (0..key.len()).collect();

        if q != key_entries { return Err(DecoderCreationError::BadKey) }

        Ok(SinglePermutationDecoded { s, key })
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        if index >= self.len() {
            return None;
        }

        let block_size = self.key.len() * self.key.len();

        let block_num = index / block_size;
        let block_position = index % block_size;

        let i = block_position / self.key.len();
        let j = block_position % self.key.len();

        let i_in_decoded = self.key[j];
        let j_in_decoded = i;

        let index = block_num * block_size + i_in_decoded * self.key.len() + j_in_decoded;

        let item = self.s.get(index).map(|x| *x).unwrap_or(0u8);

        Some(item)
    }

    pub fn len(&self) -> usize {
        let block_size = self.key.len() * self.key.len();
        let blocks_num = self.s.len() / block_size + 1;

        blocks_num * block_size
    }
}