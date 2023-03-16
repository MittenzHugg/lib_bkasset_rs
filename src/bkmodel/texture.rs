use super::super::bktexture::*;

#[derive(Debug)]
pub struct BKTextureList{
    pub texture_headers : Vec<BKTextureHeader>,
    pub texture_data: Vec<u8>,
}

impl BKTextureList{
    pub fn from_be_bytes(bytes: &[u8])->BKTextureList{
        let byte_count = u32::from_be_bytes(bytes[0..4].try_into().unwrap()) as usize;
        let count = u16::from_be_bytes(bytes[4..6].try_into().unwrap()) as usize;
        let texture_headers :Vec<_> = bytes[8..].chunks_exact(0x10).take(count)
            .map(|bytes| BKTextureHeader::from_be_bytes(bytes))
            .collect();

        let this = BKTextureList{
            texture_headers,
            texture_data: bytes[0x8 + count*0x10 .. byte_count].to_vec(),
        };
        #[cfg(feature = "test_byte_matching")]assert_eq!(bytes[.. byte_count], this.to_be_bytes(), "\n{:#?}", this);
        return this
    }

    pub fn to_be_bytes(&self)->Vec<u8>{
        let byte_count = self.texture_data.len() + self.texture_headers.len()*0x10 + 0x8;
        let count  = self.texture_headers.len();
        (byte_count as u32).to_be_bytes().into_iter()
            .chain((count as u16).to_be_bytes().into_iter())
            .chain([0,0].into_iter())
            .chain(self.texture_headers.iter().flat_map(|hdr| hdr.to_be_bytes()))
            .chain(self.texture_data.clone().into_iter())
            .collect()
    }

    pub fn size(&self)->usize{
        8 + 0x10*self.texture_headers.len() + self.texture_data.len()
    }
}