/* base type */
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct I8{
    pub i: u8,
}

/* ADAPTOR ITERATOR */
pub struct I8Adaptor<I>{
    iter: I,
}

impl<I> Iterator for I8Adaptor<I>
where
    I: Iterator<Item = u8>
{
    type Item = I8;
    fn next(&mut self) -> Option<Self::Item> {
        Some(I8{
            i: self.iter.next()?,
        })
    }
}

impl<I> I8Adaptor<I>{
    pub fn new(iter: I)->Self{
        Self{iter}
    }
}

pub trait I8Iterator: Iterator<Item = u8> + Sized{
    fn i8_iter(self) -> I8Adaptor<Self> {
        I8Adaptor::new(self)
    }
}

impl<I: Iterator<Item = u8>> I8Iterator for I {}

/* TESTS */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn i8_from_bytes() {
        let input : Vec<u8> = vec![0, 1, 2];
        let mut i8_iter = input.into_iter().i8_iter();
        
        assert_eq!(Some(I8{i:0}), i8_iter.next());
        assert_eq!(Some(I8{i:1}), i8_iter.next());
        assert_eq!(Some(I8{i:2}), i8_iter.next());
        assert_eq!(None, i8_iter.next());
    }
}