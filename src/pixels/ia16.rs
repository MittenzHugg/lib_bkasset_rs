use super::i4::I4;
use super::color_i8::I8;
use super::ia4::IA4;
use super::ia8::IA8;
use super::rgba16::RGBA16;
use super::rgba32::RGBA32;

/* base type */
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct IA16{
    pub i: u8,
    pub a: u8
}

impl IA16{
    pub fn from_be_bytes(bytes: &[u8; 2])->Self{
        return Self{i: bytes[0], a: bytes[1]}
    }

    pub fn to_be_bytes(&self)->[u8; 2]{
        return [self.i, self.a]
    }
}

impl From<I4> for IA16{ fn from(i4: I4) -> Self{IA16::from(&i4)}}
impl From<&I4> for IA16{
    fn from(i4: &I4) -> Self {
        let i = (i4.i << 4) | i4.i;
        IA16 { 
            i, 
            a: 0xFF, 
        }
    }
}

impl From<I8> for IA16{ fn from(color_i8: I8) -> Self{IA16::from(&color_i8)}}
impl From<&I8> for IA16{
    fn from(color_i8: &I8) -> Self {
        IA16 { 
            i: color_i8.i,
            a: 0xFF, 
        }
    }
}

impl From<IA4> for IA16{ fn from(ia4: IA4) -> Self{IA16::from(&ia4)}}
impl From<&IA4> for IA16{
    fn from(ia4: &IA4) -> Self {
        let i = (ia4.i << 5) | (ia4.i << 2) | (ia4.i >> 1);
        IA16 { 
            i, 
            a: match ia4.a {0 => 0, _ => 0xFF}, 
        }
    }
}

impl From<IA8> for IA16{ fn from(ia8: IA8) -> Self{IA16::from(&ia8)}}
impl From<&IA8> for IA16{
    fn from(ia8: &IA8) -> Self {
        let i = (ia8.i << 4) | ia8.i;
        IA16 { 
            i: (ia8.i << 4) | ia8.i,
            a: (ia8.a << 4) | ia8.a, 
        }
    }
}

impl From<RGBA16>  for IA16{ fn from(rgba16: RGBA16) -> Self{IA16::from(&rgba16)}}
impl From<&RGBA16> for IA16{ fn from(rgba16: &RGBA16) -> Self{IA16::from(&RGBA32::from(rgba16))}}
impl From<RGBA32>  for IA16{ fn from(rgba32: RGBA32) -> Self{IA16::from(&rgba32)}}
impl From<&RGBA32> for IA16{
    fn from(rgba32: &RGBA32) -> Self {
        IA16 { 
            i: (((rgba32.r as u16) + (rgba32.g as u16) + (rgba32.b as u16))/3) as u8,
            a: rgba32.a, 
        }
    }
}

/* ADAPTOR ITERATOR */
pub struct IA16Adaptor<I>{
    iter: I,
}

impl<I> Iterator for IA16Adaptor<I>
where
    I: Iterator<Item = u8>
{
    type Item = IA16;
    fn next(&mut self) -> Option<Self::Item> {
        Some(IA16{
            i: self.iter.next()?,
            a: self.iter.next()?,
        })
    }
}

impl<I> IA16Adaptor<I>{
    pub fn new(iter: I)->Self{
        Self{iter}
    }
}

pub trait IA16Iterator: Iterator<Item = u8> + Sized{
    fn ia16_iter(self) -> IA16Adaptor<Self> {
        IA16Adaptor::new(self)
    }
}

impl<I: Iterator<Item = u8>> IA16Iterator for I {}

/* TESTS */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ia16_from_bytes() {
        let input : Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        let mut ia16_iter = input.into_iter().ia16_iter();
        
        assert_eq!(Some(IA16{i:0, a:1}), ia16_iter.next());
        assert_eq!(Some(IA16{i:2, a:3}), ia16_iter.next());
        assert_eq!(Some(IA16{i:4, a:5}), ia16_iter.next());
        assert_eq!(None, ia16_iter.next());
    }
}