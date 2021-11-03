use crate::encode::DoublePermutationEncoded;

pub struct EncodedIter<'a, 'b, 'c> {
    encoded: DoublePermutationEncoded<'a, 'b, 'c>,
    pos: usize
}

impl<'a, 'b, 'c> Iterator for EncodedIter<'a, 'b, 'c> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.encoded.len() {
            let item = self.encoded.get(self.pos);
            self.pos += 1;
            item
        } else {
            None
        }
    }
}

impl<'a, 'b, 'c> IntoIterator for DoublePermutationEncoded<'a, 'b, 'c> {
    type Item = u8;
    type IntoIter = EncodedIter<'a, 'b, 'c>;

    fn into_iter(self) -> Self::IntoIter {
        EncodedIter { pos: 0, encoded: self }
    }
}
