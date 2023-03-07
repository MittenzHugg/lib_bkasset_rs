use super::i4::I4;
use super::color_i8::I8;
use super::ia4::IA4;
use super::ia8::IA8;
use super::ia16::IA16;
use super::rgba16::RGBA16;

#[derive(Debug, PartialEq, Clone)]
pub struct RGBA32{
    pub r:u8,
    pub g:u8,
    pub b:u8,
    pub a:u8
}

impl RGBA32{
    pub fn from_be_bytes(bytes: [u8; 4])->Self{ RGBA32{r: bytes[0], g: bytes[1], b: bytes[2], a: bytes[3]} }
    pub fn to_be_bytes(&self)->[u8; 4]{ [self.r, self.g, self.b, self.a] }
}

impl From<I4> for RGBA32{ fn from(i4: I4) -> Self{RGBA32::from(&i4)}}
impl From<&I4> for RGBA32{
    fn from(i4: &I4) -> Self {
        let i = (i4.i << 4) | i4.i;
        RGBA32 { 
            r: i, 
            g: i, 
            b: i, 
            a: 0xFF, 
        }
    }
}
impl From<I8> for RGBA32{ fn from(color_i8: I8) -> Self{RGBA32::from(&color_i8)}}
impl From<&I8> for RGBA32{
    fn from(color_i8: &I8) -> Self {
        RGBA32 { 
            r: color_i8.i, 
            g: color_i8.i, 
            b: color_i8.i, 
            a: 0xFF, 
        }
    }
}

impl From<IA4> for RGBA32{ fn from(ia4: IA4) -> Self{RGBA32::from(&ia4)}}
impl From<&IA4> for RGBA32{
    fn from(ia4: &IA4) -> Self {
        let i = (ia4.i << 5) | (ia4.i << 2) | (ia4.i >> 1);
        RGBA32 { 
            r: i, 
            g: i, 
            b: i, 
            a: match ia4.a { 0 => 0, _ => 0xFF}, 
        }
    }
}

impl From<IA8> for RGBA32{ fn from(ia8: IA8) -> Self{RGBA32::from(&ia8)}}
impl From<&IA8> for RGBA32{
    fn from(ia8: &IA8) -> Self {
        let i = (ia8.i << 4) | ia8.i;
        RGBA32 { 
            r: i, 
            g: i, 
            b: i, 
            a: (ia8.a << 4) | ia8.a, 
        }
    }
}

impl From<IA16> for RGBA32{ fn from(ia16: IA16) -> Self{RGBA32::from(&ia16)}}
impl From<&IA16> for RGBA32{
    fn from(ia16: &IA16) -> Self {
        RGBA32 { 
            r: ia16.i, 
            g: ia16.i, 
            b: ia16.i, 
            a: ia16.a, 
        }
    }
}

impl From<RGBA16> for RGBA32{ fn from(rgba16: RGBA16) -> Self{RGBA32::from(&rgba16)}}
impl From<&RGBA16> for RGBA32{
    fn from(rgba16: &RGBA16) -> Self {
        RGBA32 { 
            r: (rgba16.r << 3) | (rgba16.r >> 2), 
            g: (rgba16.g << 3) | (rgba16.g >> 2), 
            b: (rgba16.b << 3) | (rgba16.b >> 2), 
            a: match rgba16.a {0 => 0, _ => 0xFF}, 
        }
    }
}

pub struct RGBA32Adaptor<I>{
    iter: I,
}

impl<I> Iterator for RGBA32Adaptor<I>
where
    I: Iterator<Item = u8>
{
    type Item = RGBA32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(RGBA32{
            r: self.iter.next()?,
            g: self.iter.next()?,
            b: self.iter.next()?,
            a: self.iter.next()?,
        })
    }
}

impl<I> RGBA32Adaptor<I>{
    pub fn new(iter: I)->Self{
        Self{iter}
    }
}

pub trait RGBA32Iterator: Iterator<Item = u8> + Sized{
    fn rgba32_iter(self) -> RGBA32Adaptor<Self> {
        RGBA32Adaptor::new(self)
    }
}

impl<I: Iterator<Item = u8>> RGBA32Iterator for I {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rgba32_from_bytes() {
        let input : Vec<u8> = vec![0,1,2,3,4,5,6,7,8,9,0xa,0xb];
        let mut rgba32_iter = input.into_iter().rgba32_iter();
        
        assert_eq!(Some(RGBA32{r:0, g: 1, b: 2, a: 3}), rgba32_iter.next());
        assert_eq!(Some(RGBA32{r:4, g: 5, b: 6, a: 7}), rgba32_iter.next());
        assert_eq!(Some(RGBA32{r:8, g: 9, b: 10, a: 11}), rgba32_iter.next());
        assert_eq!(None, rgba32_iter.next());
    }
}
