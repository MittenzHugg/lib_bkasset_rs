#[derive(PartialEq, Debug, Clone, Copy)]
pub struct CI8(usize);

pub struct CI8Adaptor<I>{
    iter: I,
}

impl<I> Iterator for CI8Adaptor<I>
where
    I: Iterator<Item = u8>
{
    type Item = CI8;
    fn next(&mut self) -> Option<Self::Item> {
        Some(CI8(self.iter.next()? as usize))
    }
}

impl<I> CI8Adaptor<I>{
    pub fn new(iter: I)->Self{
        Self{iter}
    }
}

pub trait CI8Iterator: Iterator<Item = u8> + Sized{
    fn ci8_iter(self) -> CI8Adaptor<Self> {
        CI8Adaptor::new(self)
    }
}

impl<I: Iterator<Item = u8>> CI8Iterator for I {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ci8_from_bytes() {
        let input : Vec<u8> = vec![0, 1, 2, 3];
        let mut ci8_iter = input.into_iter().ci8_iter();
        
        assert_eq!(Some(CI8(0)), ci8_iter.next());
        assert_eq!(Some(CI8(1)), ci8_iter.next());
        assert_eq!(Some(CI8(2)), ci8_iter.next());
        assert_eq!(Some(CI8(3)), ci8_iter.next());
        assert_eq!(None, ci8_iter.next());
    }
}