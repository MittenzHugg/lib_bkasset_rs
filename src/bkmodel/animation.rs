use std::error::Error;
use super::super::error::{*};

#[derive(Debug)]
#[repr(C)]
pub struct BKAnimation{
    unk_0: [f32; 3], //position
    bone_id: i16, //bone_id
    mtx_id: i16, //parent_transform
}

impl BKAnimation{
    pub fn from_be_bytes(bytes: [u8; 0x10])->Self{
        let this = Self{
            unk_0: bytes[0..0xC].chunks_exact(4).map(|b| f32::from_be_bytes(b.try_into().unwrap())).collect::<Vec<f32>>().try_into().unwrap(),
            bone_id: i16::from_be_bytes([bytes[0xC], bytes[0xD]]),
            mtx_id: i16::from_be_bytes([bytes[0xE], bytes[0xF]]),
        };

        #[cfg(feature = "test_byte_matching")]assert_eq!(bytes, this.to_be_bytes(), "\n{:#?}", this);
        return this

    }

    pub fn to_be_bytes(&self)->[u8; 0x10]{
        self.unk_0.into_iter().flat_map(f32::to_be_bytes)
            .chain(self.bone_id.to_be_bytes().into_iter())
            .chain(self.mtx_id.to_be_bytes().into_iter())
            .collect::<Vec<u8>>().try_into().unwrap()
    }
}

#[derive(Debug)]
pub struct BKAnimationList{
    pub unk_0: f32,
    pub animations: Vec<BKAnimation>
}

impl BKAnimationList{
    pub fn try_from_be_bytes(bytes: &[u8])->Result<Self, Box<dyn Error>>{
        let unk_0 = f32::from_be_bytes(bytes[0..4].try_into()?);
        let count = u16::from_be_bytes([bytes[4], bytes[5]]) as usize;
        let this = Self{
            unk_0,
            animations: bytes[8..].chunks_exact(0x10)
                .map(|b| b.try_into().map(|b| BKAnimation::from_be_bytes(b)))
                .take(count)
                .collect::<Result<Vec<_>, _>>()?
        };

        if this.animations.len() != count{ return Err(Box::new(TryFromBEBytesError)) }
        #[cfg(feature = "test_byte_matching")]assert_eq!(bytes[..this.size()], this.to_be_bytes(), "\n{:#?}", this);

        return Ok(this)
    }

    pub fn to_be_bytes(&self)->Vec<u8>{
        let mut bytes = Vec::with_capacity(self.size()); 
        bytes = self.unk_0.to_be_bytes().into_iter()
            .chain((self.animations.len() as u16).to_be_bytes().into_iter())
            .chain([0, 0].into_iter())
            .chain(self.animations.iter().flat_map(BKAnimation::to_be_bytes))
            .collect();
        bytes.resize(self.size(), 0);
        return bytes

    }

    pub fn size(&self)->usize{
        let size = 8 + self.animations.len()*std::mem::size_of::<BKAnimation>();
        (size + 7) & !7 //align 
    }
}