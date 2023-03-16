use std::error::Error;
use std::ops::{Deref, DerefMut};
use super::super::error::{*};

#[derive(Debug)]
pub struct BKModelUnk28{
    pub coord: [i16; 3],
    pub anim_indx: i8,
    // pub vtx_count: u8,
    pub vtx_index_list: Vec<usize>
}

impl BKModelUnk28{
    pub fn try_from_be_bytes(bytes: &[u8])->Result<Self, Box<dyn Error>>{
        let count = bytes[7] as usize;
        let this = Self{
            coord : bytes.chunks_exact(2).map(|b| i16::from_be_bytes(b.try_into().unwrap())).take(3).collect::<Vec<i16>>().try_into().unwrap(),
            anim_indx: bytes[6] as i8,
            vtx_index_list: bytes[8..].chunks_exact(2).map(|b| b.try_into().map(|b| u16::from_be_bytes(b) as usize)).take(count).collect::<Result<Vec<usize>,_>>()?
        };

        if this.vtx_index_list.len() != count{ return Err(Box::new(TryFromBEBytesError)) } //verify length

        #[cfg(feature = "test_byte_matching")]assert_eq!(bytes[..this.size()], this.to_be_bytes(), "\n{:#?}", this);
        return Ok(this)
    }

    pub fn to_be_bytes(&self)->Vec<u8>{
        [
            self.coord.into_iter().flat_map(i16::to_be_bytes).collect::<Vec<u8>>(),
            vec![self.anim_indx as u8, self.vtx_index_list.len() as u8],
            self.vtx_index_list.iter().flat_map(|indx| (*indx as u16).to_be_bytes()).collect::<Vec<u8>>()
        ].concat()
    }

    pub fn size(&self)->usize{
        8 + self.vtx_index_list.len()*2
    }
}

#[derive(Debug)]
pub struct BKModelUnk28List{
    pub list: Vec<BKModelUnk28>
}

impl BKModelUnk28List{
    pub fn try_from_be_bytes(bytes: &[u8])->Result<Self, Box<dyn Error>>{
        let count = i16::from_be_bytes([bytes[0], bytes[1]]) as usize;
        let mut this = Self{
            list: Vec::new(),
        };
        let mut offset = 0x4;
        for _ in 0 .. count{
            this.list.push(BKModelUnk28::try_from_be_bytes(&bytes[offset..])?);
            offset += this.list.last().unwrap().size()
        }

        #[cfg(feature = "test_byte_matching")]assert_eq!(bytes[.. this.size()], this.to_be_bytes(), "\n{:#?}", this);
        return Ok(this)
    }

    pub fn to_be_bytes(&self)->Vec<u8>{
        let mut bytes = Vec::with_capacity(self.size());
        bytes = [
            (self.list.len() as i16).to_be_bytes().as_slice(),
            &[0, 0],
            self.list.iter().flat_map(BKModelUnk28::to_be_bytes).collect::<Vec<u8>>().as_slice()
        ].concat();
        bytes.resize(self.size(), 0);
        return bytes
    }

    pub fn size(&self)->usize{
        let size = 4 + self.list.iter().map(|elem| elem.size()).sum::<usize>();
        (size + 7) & !7
    }
}

impl Deref for BKModelUnk28List{
    type Target = Vec<BKModelUnk28>;
    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

impl DerefMut for BKModelUnk28List{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list
    }
}