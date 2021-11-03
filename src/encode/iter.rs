use crate::encode::SinglePermutationEncoded;

pub struct EncodedIter<'a, 'b> {
    encoded: SinglePermutationEncoded<'a, 'b>,
    pos: usize
}

impl<'a, 'b, 'c> Iterator for EncodedIter<'a, 'b> {
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

impl<'a, 'b, 'c> IntoIterator for SinglePermutationEncoded<'a, 'b> {
    type Item = u8;
    type IntoIter = EncodedIter<'a, 'b>;

    fn into_iter(self) -> Self::IntoIter {
        EncodedIter { pos: 0, encoded: self }
    }
}
