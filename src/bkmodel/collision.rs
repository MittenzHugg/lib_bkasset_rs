use std::error::Error;
use std::fmt;

use super::super::error::TryFromBEBytesError;

#[repr(C)]
pub struct BKCollisionTri{
    pub vtx: [i16; 3],
    pub unk_6: i16,
    pub flags: u32,
}

impl BKCollisionTri{
    pub fn from_be_bytes(bytes: [u8; 0xC]) -> BKCollisionTri{
        let vtx = [
            i16::from_be_bytes([bytes[0], bytes[1]]), 
            i16::from_be_bytes([bytes[2], bytes[3]]),
            i16::from_be_bytes([bytes[4], bytes[5]])
        ];
        let unk_6 = i16::from_be_bytes([bytes[6], bytes[7]]);
        let flags = u32::from_be_bytes(bytes[8..0xC].try_into().unwrap());
        let this = BKCollisionTri{vtx, unk_6, flags};
        #[cfg(feature = "test_byte_matching")]assert_eq!(bytes, this.to_be_bytes(), "\n{:#?}", this);
        return this
    }

    pub fn to_be_bytes(&self)->[u8; 0xC]{
        [
            self.vtx.iter().flat_map(|val| val.to_be_bytes()).collect::<Vec<_>>().as_slice(),
            self.unk_6.to_be_bytes().as_slice(),
            self.flags.to_be_bytes().as_slice(),
        ].concat().try_into().unwrap()
    }
}
impl fmt::Debug for BKCollisionTri{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BKCollisionTri: {{vtx: {:>4?}, unk_16: 0x{:03X}, flags: 0x{:03X}}}", self.vtx, self.unk_6, self.flags)
    }
}

#[repr(C)]

pub struct BKCollisionMesh{
    tri_start: i16,
    size: i16,
}

impl BKCollisionMesh{
    pub fn from_be_bytes(bytes: [u8; 4]) -> BKCollisionMesh{
        let tri_start = i16::from_be_bytes([bytes[0], bytes[1]]);
        let size = i16::from_be_bytes([bytes[2], bytes[3]]);
        let this = BKCollisionMesh{tri_start, size};
        #[cfg(feature = "test_byte_matching")]assert_eq!(bytes, this.to_be_bytes(), "\n{:#?}", this);

        return this;
    }

    pub fn to_be_bytes(&self)->[u8; 4]{
        [
            self.tri_start.to_be_bytes(),
            self.size.to_be_bytes()
        ].concat().try_into().unwrap()
    }
}

impl fmt::Debug for BKCollisionMesh{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BKCollisionMesh{{start_tri: {}, size: {}}}", self.tri_start, self.size)
    }
}

#[derive(Debug)]
pub struct BKCollisionList{
    unk_0: [i16; 3], //min
    unk_6: [i16; 3], //max
    unk_c: i16, //y_stride
    unk_e: i16, //z_stride
    unk_12: i16, //scale
    geo: Vec<BKCollisionMesh>,
    tri: Vec<BKCollisionTri>,
}

impl BKCollisionList{
    pub fn try_from_be_bytes(bytes: &[u8]) -> Result<BKCollisionList, Box<dyn Error>>{
        let mut shorts = bytes
            .chunks_exact(2)
            .map(|b| b.try_into().map(|b| i16::from_be_bytes(b)))
            .take(11)
            .collect::<Result<Vec<_>, _>>()?;
        let geo_cnt = shorts[8] as usize;
        let tri_cnt = shorts[10] as usize;
        let geo_offset = 0x18;
        let tri_offset = geo_offset + geo_cnt*4;
        let _size = tri_offset + tri_cnt*0xC;
                
        let this = BKCollisionList{
            unk_0  : [shorts[0], shorts[1], shorts[2]],
            unk_6  : [shorts[3], shorts[4], shorts[5]],
            unk_c  : shorts[6],
            unk_e  : shorts[7],
            unk_12 : shorts[9],
            geo: bytes[geo_offset..].chunks_exact(4)
                .map(|b| b.try_into().map(|b| BKCollisionMesh::from_be_bytes(b)))
                .take(geo_cnt as usize)
                .collect::<Result<Vec<_>, _>>()?,
            tri: bytes[tri_offset .. ].chunks_exact(0xC)
                .map(|b| b.try_into().map(|b| BKCollisionTri::from_be_bytes(b)))
                .take(tri_cnt as usize)
                .collect::<Result<Vec<_>, _>>()?
        };
        
        if geo_cnt != this.geo.len() { return Err(Box::new(TryFromBEBytesError)) }
        if tri_cnt != this.tri.len() { return Err(Box::new(TryFromBEBytesError)) }
        #[cfg(feature = "test_byte_matching")]assert_eq!(bytes[..this.size()], this.to_be_bytes(), "\n{:#?}", this);

        return Ok(this)
    }

    pub fn to_be_bytes(&self)->Vec<u8>{
        let mut bytes = [
            self.unk_0.iter().flat_map(|val| val.to_be_bytes()).collect::<Vec<_>>().as_slice(), //min
            self.unk_6.iter().flat_map(|val| val.to_be_bytes()).collect::<Vec<_>>().as_slice(), //max
            self.unk_c.to_be_bytes().as_slice(), //y_stride
            self.unk_e.to_be_bytes().as_slice(), //z_stride
            (self.geo.len() as i16).to_be_bytes().as_slice(), //geo_cnt
            self.unk_12.to_be_bytes().as_slice(), //scale
            (self.tri.len() as i16).to_be_bytes().as_slice(), //geo_cnt
            &[0,0],
            self.geo.iter().flat_map(|val| val.to_be_bytes()).collect::<Vec<_>>().as_slice(),
            self.tri.iter().flat_map(|val| val.to_be_bytes()).collect::<Vec<_>>().as_slice(),
        ].concat();
        bytes.resize(self.size(), 0);
        return bytes
    }

    pub fn size(&self)->usize{
        ((0x18 + 4*self.geo.len() + 0xC*self.tri.len()) + 7 ) & !7
    }
}