/* base type */
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct CI4(usize);

pub struct CI4Adaptor<I>{
    value: Option<u8>,
    iter: I,
}

impl<I> Iterator for CI4Adaptor<I>
where
    I: Iterator<Item = u8>
{
    type Item = CI4;
    fn next(&mut self) -> Option<Self::Item> {
        if self.value == None {
            self.value = self.iter.next();
            return Some(CI4(((self.value? >> 4) & 0x0F) as usize))
        }

        let lower = CI4((self.value? & 0x0F) as usize);
        self.value = None;
        return Some(lower);
    }
}

impl<I> CI4Adaptor<I>{
    pub fn new(iter: I)->Self{
        Self{iter, value: None}
    }
}

pub trait CI4Iterator: Iterator<Item = u8> + Sized{
    fn ci4_iter(self) -> CI4Adaptor<Self> {
        CI4Adaptor::new(self)
    }
}

impl<I: Iterator<Item = u8>> CI4Iterator for I {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ci4_from_bytes() {
        let input : Vec<u8> = vec![0x01,0x23];
        let mut ci4_iter = input.into_iter().ci4_iter();
        
        assert_eq!(Some(CI4(0)), ci4_iter.next());
        assert_eq!(Some(CI4(1)), ci4_iter.next());
        assert_eq!(Some(CI4(2)), ci4_iter.next());
        assert_eq!(Some(CI4(3)), ci4_iter.next());
        assert_eq!(None, ci4_iter.next());
    }
}