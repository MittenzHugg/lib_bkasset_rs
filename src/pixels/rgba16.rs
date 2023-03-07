use super::i4::I4;
use super::color_i8::I8;
use super::ia4::IA4;
use super::ia8::IA8;
use super::ia16::IA16;
use super::rgba32::RGBA32;

/* base type */
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct RGBA16{
    pub r:u8,
    pub g:u8,
    pub b:u8,
    pub a:u8
}

impl RGBA16{
    pub fn from_be_bytes(bytes: [u8; 2])->Self{ 
        let bits = u16::from_be_bytes([bytes[0], bytes[1]]);

        RGBA16{
            r: ((bits >> 11) & 0x1F) as u8,
            g: ((bits >>  6) & 0x1F) as u8,
            b: ((bits >>  1) & 0x1F) as u8,
            a: ((bits >>  0) &  0x1) as u8,
        } 
    }
    pub fn to_be_bytes(&self)->[u8; 2]{ 
        let val = (((self.r as u16) & 0x1F) << 11) | (((self.g as u16) & 0x1F) << 5) | (((self.b as u16) & 0x1F) << 1) | (self.a as u16) & 1;
        val.to_be_bytes()
    }
}

impl From<I4> for RGBA16{ fn from(i4: I4) -> Self{RGBA16::from(&i4)}}
impl From<&I4> for RGBA16{
    fn from(i4: &I4) -> Self {
        let i = (i4.i << 2) | (i4.i >> 1);
        RGBA16 { 
            r: i, 
            g: i, 
            b: i, 
            a: 1, 
        }
    }
}

impl From<I8> for RGBA16{ fn from(color_i8: I8) -> Self{RGBA16::from(&color_i8)}}
impl From<&I8> for RGBA16{
    fn from(color_i8: &I8) -> Self {
        let i = (color_i8.i << 1) | (color_i8.i >> 3);
        RGBA16 { 
            r: i, 
            g: i, 
            b: i, 
            a: 1, 
        }
    }
}

impl From<IA4> for RGBA16{ fn from(ia4: IA4) -> Self{RGBA16::from(&ia4)}}
impl From<&IA4> for RGBA16{
    fn from(ia4: &IA4) -> Self {
        let i = (ia4.i << 2) | (ia4.i >> 1);
        RGBA16 { 
            r: i, 
            g: i, 
            b: i, 
            a: ia4.a, 
        }
    }
}

impl From<IA8> for RGBA16{ fn from(ia8: IA8) -> Self{RGBA16::from(&ia8)}}
impl From<&IA8> for RGBA16{
    fn from(ia8: &IA8) -> Self {
        let i = (ia8.i << 1) | (ia8.i >> 3);
        RGBA16 { 
            r: i, 
            g: i, 
            b: i, 
            a: match ia8.a {0 => 0, _ => 1}, 
        }
    }
}

impl From<IA16> for RGBA16{ fn from(ia16: IA16) -> Self{RGBA16::from(&ia16)}}
impl From<&IA16> for RGBA16{
    fn from(ia16: &IA16) -> Self {
        let i = ia16.i >> 3;
        RGBA16 { 
            r: i, 
            g: i, 
            b: i, 
            a: match ia16.a {0 => 0, _ => 1}, 
        }
    }
}

impl From<RGBA32> for RGBA16{ fn from(rgba32: RGBA32) -> Self{RGBA16::from(&rgba32)}}
impl From<&RGBA32> for RGBA16{
    fn from(rgba32: &RGBA32) -> Self {
        RGBA16 { 
            r: rgba32.r >> 3, 
            g: rgba32.g >> 3, 
            b: rgba32.b >> 3, 
            a: match rgba32.a {0 => 0, _ => 1}, 
        }
    }
}

pub struct RGBA16Adaptor<I>{
    iter: I,
}

impl<I> Iterator for RGBA16Adaptor<I>
where
    I: Iterator<Item = u8>
{
    type Item = RGBA16;
    fn next(&mut self) -> Option<Self::Item> {
        Some(RGBA16::from_be_bytes([self.iter.next()?, self.iter.next()?]))

    }
}

impl<I> RGBA16Adaptor<I>{
    pub fn new(iter: I)->Self{
        Self{iter}
    }
}

pub trait RGBA16Iterator: Iterator<Item = u8> + Sized{
    fn rgba16_iter(self) -> RGBA16Adaptor<Self> {
        RGBA16Adaptor::new(self)
    }
}

impl<I: Iterator<Item = u8>> RGBA16Iterator for I {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rgba16_from_bytes() {
        let input : Vec<u8> = vec![0,1,2,3,4,5];
        let mut rgba16_iter = input.into_iter().rgba16_iter();
        
        assert_eq!(Some(RGBA16{r:0, g: 0, b: 0, a: 1}), rgba16_iter.next());
        assert_eq!(Some(RGBA16{r:0, g: 8, b: 1, a: 1}), rgba16_iter.next());
        assert_eq!(Some(RGBA16{r:0, g: 16, b: 2, a: 1}), rgba16_iter.next());
        assert_eq!(None, rgba16_iter.next());
    }
}