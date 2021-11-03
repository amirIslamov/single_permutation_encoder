use crate::encode::error::EncoderCreationError;

pub(crate) mod error;
pub(crate) mod iter;

use std::collections;

pub struct DoublePermutationEncoded<'a, 'b, 'c> {
    s: &'a [u8],
    hkey: &'b [usize],
    vkey: &'c [usize]
}

impl<'a, 'b, 'c> DoublePermutationEncoded<'a, 'b, 'c> {
    pub fn new(s: &'a [u8], hkey: &'b [usize], vkey: &'c [usize]) -> Result<Self, EncoderCreationError> {
        if hkey.len() == 0 { return Err(EncoderCreationError::EmptyKey) }
        if vkey.len() == 0 { return Err(EncoderCreationError::EmptyKey) }

        let hkey_entries: collections::HashSet<usize> = hkey.iter().map(|r| *r).collect();
        let vkey_entries: collections::HashSet<usize> = vkey.iter().map(|r| *r).collect();
        let hq: collections::HashSet<usize> = (0..hkey.len()).collect();
        let vq: collections::HashSet<usize> = (0..vkey.len()).collect();

        if hq != hkey_entries { return Err(EncoderCreationError::BadKey) }
        if vq != vkey_entries { return Err(EncoderCreationError::BadKey) }

        Ok(DoublePermutationEncoded { s, hkey, vkey })
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        if index >= self.len() {
            return None
        }

        let block_size = self.hkey.len() * self.vkey.len();

        let block_num = index / block_size;
        let block_position = index % block_size;

        let i = block_position / self.hkey.len();
        let j = block_position % self.hkey.len();

        let i_in_decoded = self.hkey.iter().position(|x| *x == j).unwrap();
        let j_in_decoded = self.vkey.iter().position(|x| *x == i).unwrap();

        let index = block_num * block_size + i_in_decoded * self.hkey.len() + j_in_decoded;

        let item = self.s.get(index).map(|x| *x).unwrap_or(0u8);

        Some(item)
    }

    pub fn len(&self) -> usize {
        let block_size = self.hkey.len() * self.vkey.len();
        let blocks_num = self.s.len() / block_size + 1;

        blocks_num * block_size
    }
}


