use libultra::{F3dex};
use std::ops::{Deref, DerefMut};
use std::fmt;
use std::error::Error;
use super::super::error::{*};

pub struct BKGfxList{
    pub gfx:Vec<F3dex>,
    header_filler: Option<[u8; 4]>, //used to preserve byte matching, maybe [0x00; 4] OR [0x0F; 4]
}

impl BKGfxList{
    pub fn try_from_be_bytes(bytes: &[u8])->Result<BKGfxList, Box<dyn Error>>{
        let count = u32::from_be_bytes(bytes[0..4].try_into()?) as usize;
        let bytes = &bytes[0..8+8*count];
        let this = BKGfxList{
            header_filler : Some(bytes[4..8].try_into()?),
            gfx : bytes[8..].chunks_exact(8)
                .map(<[u8; 8]>::try_from)
                .map(|b| b.map(u64::from_be_bytes).map(F3dex::from))
                .take(count)
                .collect::<Result<Vec<_>,_>>()?
        };
        
        if this.len() != count{ return Err(Box::new(TryFromBEBytesError)) } //verify length

        #[cfg(feature = "test_byte_matching")]assert_eq!(bytes, this.to_be_bytes(), "\n{:#?}", this);
        return Ok(this)
    }

    pub fn to_be_bytes(&self)->Vec<u8>{
        let len = self.len() as u32;
        let mut out : Vec<u8> = len.to_be_bytes().to_vec();
        
        match self.header_filler {
            Some(fill) => out.append(&mut fill.to_vec()),
            None => out.append(&mut vec![0;4])
        }

        out.append(&mut self.gfx.clone().into_iter()
            .map(u64::from)
            .flat_map(u64::to_be_bytes)
            .collect::<Vec<u8>>()
        );
        return out;
    }

    pub fn size(&self)->usize{
        8 + 8*self.len()
    }
}

impl Deref for BKGfxList{
    type Target = Vec<F3dex>;
    fn deref(&self) -> &Vec<F3dex> { &self.gfx }
}

impl DerefMut for BKGfxList {
    fn deref_mut(&mut self) -> &mut Vec<F3dex> { &mut self.gfx }
}

impl fmt::Debug for BKGfxList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter())
         .finish()
    }
}

