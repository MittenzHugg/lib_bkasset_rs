use std::error::Error;
use super::super::error::TryFromBEBytesError;

#[derive(Debug)]
pub struct BKModelUnk14ListHeader{
    type_0_count: usize,
    type_1_count: usize,
    type_2_count: usize,
    unk_6: i16,
}

impl BKModelUnk14ListHeader {
    pub fn from_be_bytes(bytes: [u8; 0x8])->Self{
        let this = BKModelUnk14ListHeader { 
            type_0_count: u16::from_be_bytes([bytes[0], bytes[1]]) as usize, 
            type_1_count: u16::from_be_bytes([bytes[2], bytes[3]]) as usize,
            type_2_count: u16::from_be_bytes([bytes[4], bytes[5]]) as usize,
            unk_6:  i16::from_be_bytes([bytes[6], bytes[7]])
        };
        #[cfg(feature = "test_byte_matching")]assert_eq!(bytes, this.to_be_bytes(), "\n{:#?}", this);
        return this
    }

    pub fn to_be_bytes(&self)->[u8; 0x8]{
        [
            (self.type_0_count as u16).to_be_bytes(),
            (self.type_1_count as u16).to_be_bytes(),
            (self.type_2_count as u16).to_be_bytes(),
            (self.unk_6 as u16).to_be_bytes(),
        ].concat().try_into().unwrap()
    }
}

#[derive(Debug)]
pub struct BKModelUnk14List{
    pub scale: i16,
    pub type_0_list :Vec<BKModelUnk14Type0>,
    pub type_1_list :Vec<BKModelUnk14Type1>,
    pub type_2_list :Vec<BKModelUnk14Type2>,
}

impl BKModelUnk14List {
    pub fn try_from_be_bytes(bytes: &[u8])->Result<Self, Box<dyn Error>>{
        let header_bytes: [u8; 8] = bytes[..8].try_into()?;
        let mut offset = 8;
        let header = BKModelUnk14ListHeader::from_be_bytes(header_bytes);
        
        let type_0_list: Vec<BKModelUnk14Type0> = bytes[offset..].chunks_exact(std::mem::size_of::<BKModelUnk14Type0>())
            .map(|b| b.try_into().map(|b| BKModelUnk14Type0::from_be_bytes(b)))
            .take(header.type_0_count)
            .collect::<Result<Vec<_>,_>>()?;
        offset += std::mem::size_of::<BKModelUnk14Type0>()*header.type_0_count;
        if header.type_0_count != type_0_list.len() { return Err(Box::new(TryFromBEBytesError))}

        let type_1_list: Vec<BKModelUnk14Type1> = bytes[offset..].chunks_exact(std::mem::size_of::<BKModelUnk14Type1>())
            .map(|b| b.try_into().map(|b| BKModelUnk14Type1::from_be_bytes(b)))
            .take(header.type_1_count)
            .collect::<Result<Vec<_>,_>>()?;
        offset += std::mem::size_of::<BKModelUnk14Type1>()*header.type_1_count;
        if header.type_1_count != type_1_list.len() { return Err(Box::new(TryFromBEBytesError))}

        let type_2_list: Vec<BKModelUnk14Type2> = bytes[offset..].chunks_exact(std::mem::size_of::<BKModelUnk14Type2>())
            .map(|b| b.try_into().map(|b| BKModelUnk14Type2::from_be_bytes(b)))
            .take(header.type_2_count)
            .collect::<Result<Vec<_>,_>>()?;
        if header.type_2_count != type_2_list.len() { return Err(Box::new(TryFromBEBytesError))}

        let this =  BKModelUnk14List {  
            scale : header.unk_6,
            type_0_list,
            type_1_list,
            type_2_list,
        };

        #[cfg(feature = "test_byte_matching")]assert_eq!(bytes[..this.size()], this.to_be_bytes(), "\n{:#?}", this);

        return Ok(this)
    }

    pub fn to_be_bytes(&self)->Vec<u8>{
        let mut out = [
            self.get_header().to_be_bytes().to_vec(),
            self.type_0_list.iter().flat_map(BKModelUnk14Type0::to_be_bytes).collect(),
            self.type_1_list.iter().flat_map(BKModelUnk14Type1::to_be_bytes).collect(),
            self.type_2_list.iter().flat_map(BKModelUnk14Type2::to_be_bytes).collect(),
        ].concat();
        out.resize(self.size(), 0);
        return out
    }

    pub fn get_header(&self)->BKModelUnk14ListHeader{
        BKModelUnk14ListHeader{
            type_0_count: self.type_0_list.len(),
            type_1_count: self.type_1_list.len(),
            type_2_count: self.type_2_list.len(),
            unk_6: self.scale,
        }
    }

    pub fn size(&self)->usize{
        ((0x8 + self.type_0_list.len()*0x18 + self.type_1_list.len()*0x10 + self.type_2_list.len()*0xC) + 7) & !7
    }
}


#[derive(Debug)]
#[repr(C)]
pub struct BKModelUnk14Type0{
    pub unk_0:  [i16; 3],
    pub unk_6:  [i16; 3],
    pub unk_c:  [i16; 3],
    pub unk_12: [u8; 3],
    pub unk_15: u8,
    pub unk_16: i8,
}

impl BKModelUnk14Type0 {
    pub fn from_be_bytes(bytes: [u8; 0x18])->Self{
        let mut short_iter = bytes.chunks_exact(2).map(|bytes| i16::from_be_bytes(bytes.try_into().unwrap()));

        let this = BKModelUnk14Type0{
            unk_0: [short_iter.next().unwrap().clone(), short_iter.next().unwrap().clone(), short_iter.next().unwrap().clone()],
            unk_6: [short_iter.next().unwrap().clone(), short_iter.next().unwrap().clone(), short_iter.next().unwrap().clone()],
            unk_c: [short_iter.next().unwrap().clone(), short_iter.next().unwrap().clone(), short_iter.next().unwrap().clone()],
            unk_12: [bytes[0x12], bytes[0x13], bytes[0x14]],
            unk_15: bytes[0x15],
            unk_16: bytes[0x16] as i8,
        };

        #[cfg(feature = "test_byte_matching")]assert_eq!(bytes, this.to_be_bytes(), "\n{:#?}", this);

        return this
    }

    pub fn to_be_bytes(&self)->[u8; 0x18]{
        [
            self.unk_0.into_iter().flat_map(i16::to_be_bytes).collect(),
            self.unk_6.into_iter().flat_map(i16::to_be_bytes).collect(),
            self.unk_c.into_iter().flat_map(i16::to_be_bytes).collect(),
            self.unk_12.to_vec(),
            vec![self.unk_15, self.unk_16 as u8, 0]
        ].concat().try_into().unwrap()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct BKModelUnk14Type1{
    pub raw_bytes: [u8; 0x10],
}

impl BKModelUnk14Type1 {
    pub fn from_be_bytes(bytes: [u8; 0x10])->Self{
        let this = BKModelUnk14Type1{
            raw_bytes: bytes
        };

        #[cfg(feature = "test_byte_matching")]assert_eq!(bytes, this.to_be_bytes(), "\n{:#?}", this);
        return this
    }

    pub fn to_be_bytes(&self)->[u8; 0x10]{
        self.raw_bytes
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct BKModelUnk14Type2{
    pub raw_bytes: [u8; 0xC],
}

impl BKModelUnk14Type2 {
    pub fn from_be_bytes(bytes: [u8; 0xC])->Self{
        let this = BKModelUnk14Type2{
            raw_bytes: bytes
        };
        
        #[cfg(feature = "test_byte_matching")]assert_eq!(bytes, this.to_be_bytes(), "\n{:#?}", this);
        return this
    }

    pub fn to_be_bytes(&self)->[u8; 0xC]{
        self.raw_bytes
    }
}
