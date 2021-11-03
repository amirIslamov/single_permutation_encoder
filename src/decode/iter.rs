use crate::decode::SinglePermutationDecoded;

pub struct DecodedIter<'a, 'b> {
    decoded: SinglePermutationDecoded<'a, 'b>,
    pos: usize
}

impl<'a, 'b, 'c> Iterator for DecodedIter<'a, 'b> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.decoded.len() {
            let item = self.decoded.get(self.pos);
            self.pos += 1;
            item
        } else {
            None
        }
    }
}

impl<'a, 'b, 'c> IntoIterator for SinglePermutationDecoded<'a, 'b> {
    type Item = u8;
    type IntoIter = DecodedIter<'a, 'b>;

    fn into_iter(self) -> Self::IntoIter {
        DecodedIter { pos: 0, decoded: self }
    }
}