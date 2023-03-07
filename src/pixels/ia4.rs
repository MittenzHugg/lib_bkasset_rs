/* base type */
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct IA4{
    pub i: u8,
    pub a: u8
}

/* ADAPTOR ITERATOR */
pub struct IA4Adaptor<I>{
    buffer: Option<u8>,
    iter: I,
}

impl<I> Iterator for IA4Adaptor<I>
where
    I: Iterator<Item = u8>
{
    type Item = IA4;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer == None {
            self.buffer = self.iter.next();
            let bits = (self.buffer? >> 4) & 0x0F;
            return Some(IA4{i: (bits >> 1) & 0x7, a: bits & 1})
        }

        let bits = self.buffer? & 0x0F;
        self.buffer = None;
        return Some(IA4{i: (bits >> 1) & 0x7, a: bits & 1})
    }
}

impl<I> IA4Adaptor<I>{
    pub fn new(iter: I)->Self{
        Self{iter, buffer: None}
    }
}

pub trait IA4Iterator: Iterator<Item = u8> + Sized{
    fn ia4_iter(self) -> IA4Adaptor<Self> {
        IA4Adaptor::new(self)
    }
}

impl<I: Iterator<Item = u8>> IA4Iterator for I {}

/* TESTS */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ia4_from_bytes() {
        let input : Vec<u8> = vec![0x14, 0x78];
        let mut ia4_iter = input.into_iter().ia4_iter();
        
        assert_eq!(Some(IA4{i:0, a:1}), ia4_iter.next());
        assert_eq!(Some(IA4{i:2, a:0}), ia4_iter.next());
        assert_eq!(Some(IA4{i:3, a:1}), ia4_iter.next());
        assert_eq!(Some(IA4{i:4, a:0}), ia4_iter.next());
        assert_eq!(None, ia4_iter.next());
    }
}
