/* base type */
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct I4{
    pub i: u8,
}

/* ADAPTOR ITERATOR */
pub struct I4Adaptor<I>{
    buffer: Option<u8>,
    iter: I,
}

impl<I> Iterator for I4Adaptor<I>
where
    I: Iterator<Item = u8>
{
    type Item = I4;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer == None {
            self.buffer = self.iter.next();
            return Some(I4{i: (self.buffer? >> 4) & 0x0F})
        }

        let bits = self.buffer? & 0x0F;
        self.buffer = None;
        return Some(I4{i: bits})
    }
}

impl<I> I4Adaptor<I>{
    pub fn new(iter: I)->Self{
        Self{iter, buffer: None}
    }
}

pub trait I4Iterator: Iterator<Item = u8> + Sized{
    fn i4_iter(self) -> I4Adaptor<Self> {
        I4Adaptor::new(self)
    }
}

impl<I: Iterator<Item = u8>> I4Iterator for I {}

/* TESTS */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn i4_from_bytes() {
        let input : Vec<u8> = vec![0x01, 0x23];
        let mut i4_iter = input.into_iter().i4_iter();
        
        assert_eq!(Some(I4{i:0}), i4_iter.next());
        assert_eq!(Some(I4{i:1}), i4_iter.next());
        assert_eq!(Some(I4{i:2}), i4_iter.next());
        assert_eq!(Some(I4{i:3}), i4_iter.next());
        assert_eq!(None, i4_iter.next());
    }
}
