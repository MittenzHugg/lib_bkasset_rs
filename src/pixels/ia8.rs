/* base type */
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct IA8{
    pub i: u8,
    pub a: u8
}

/* ADAPTOR ITERATOR */
pub struct IA8Adaptor<I>{
    iter: I,
}

impl<I> Iterator for IA8Adaptor<I>
where
    I: Iterator<Item = u8>
{
    type Item = IA8;
    fn next(&mut self) -> Option<Self::Item> {
        let bits = self.iter.next()?;
        Some(IA8{
            i: (bits >> 4) & 0x0F,
            a: bits & 0x0F,
        })
    }
}

impl<I> IA8Adaptor<I>{
    pub fn new(iter: I)->Self{
        Self{iter}
    }
}

pub trait IA8Iterator: Iterator<Item = u8> + Sized{
    fn ia8_iter(self) -> IA8Adaptor<Self> {
        IA8Adaptor::new(self)
    }
}

impl<I: Iterator<Item = u8>> IA8Iterator for I {}

/* TESTS */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ia8_from_bytes() {
        let input : Vec<u8> = vec![0x01, 0x23, 0x45];
        let mut ia8_iter = input.into_iter().ia8_iter();
        
        assert_eq!(Some(IA8{i:0, a:1}), ia8_iter.next());
        assert_eq!(Some(IA8{i:2, a:3}), ia8_iter.next());
        assert_eq!(Some(IA8{i:4, a:5}), ia8_iter.next());
        assert_eq!(None, ia8_iter.next());
    }
}
