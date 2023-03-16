use std::error::Error;
use super::super::error::TryFromBEBytesError;

pub struct BKModelUnk20ListHeader{
    count: u8,
    pad_1: u8,
}

impl BKModelUnk20ListHeader{
    pub fn from_be_bytes(bytes: [u8; 2])->Self{
        Self{
            count: bytes[0],
            pad_1: bytes[1]
        }
    }

    pub fn to_be_bytes(&self)->[u8; 2]{
        [self.count, self.pad_1]
    }
}

#[derive(Debug)]
pub struct BKModelUnk20List{
    unk_20_list: Vec<BKModelUnk20Element>
}

impl BKModelUnk20List{
    pub fn try_from_be_bytes(bytes: &[u8])->Result<Self, Box<dyn Error>>{
        let header = BKModelUnk20ListHeader::from_be_bytes([bytes[0], bytes[1]]);
        let this = Self{
            unk_20_list : bytes[2..].chunks_exact(0xE)
            .map(<[u8;0xE]>::try_from)
            .map(|b| b.map(BKModelUnk20Element::from_be_bytes))
            .take(header.count as usize)
            .collect::<Result<Vec<_>,_>>()?
        };        
        if this.unk_20_list.len() != header.count as usize { return Err(Box::new(TryFromBEBytesError)) }
        
        #[cfg(feature = "test_byte_matching")]assert_eq!(bytes[..this.size()], this.to_be_bytes(), "\n{:#?}", this);
        Ok(this)
    }

    pub fn to_be_bytes(&self)->Vec<u8>{
        let mut bytes = Vec::with_capacity(self.size());
        bytes = [
            [self.unk_20_list.len() as u8 ,0].as_slice(),
            self.unk_20_list.iter()
                .flat_map(BKModelUnk20Element::to_be_bytes)
                .collect::<Vec<_>>().as_slice()
        ].concat();
        bytes.resize(self.size(), 0);
        return bytes
    }

    pub fn size(&self)->usize{
        let size = 2 + self.unk_20_list.len()*0xE;
        (size  + 7) & !7 //align
    }
}

#[derive(Debug)]
pub struct BKModelUnk20Element{
        unk_0: [i16; 3],
        unk_6: [i16; 3],
        unk_c: u8,
}

impl BKModelUnk20Element{
    pub fn from_be_bytes(bytes: [u8; 0xE])->Self{
        let mut shorts : Vec<i16> = bytes.chunks_exact(2)
            .map(|b| i16::from_be_bytes(b.try_into().unwrap()))
            .take(6)
            .collect();

        let this = Self{
            unk_0 : [shorts[0], shorts[1], shorts[2]],
            unk_6 : [shorts[3], shorts[4], shorts[5]],
            unk_c : bytes[0xC],
        };

        #[cfg(feature = "test_byte_matching")]assert_eq!(bytes, this.to_be_bytes(), "\n{:#?}", this);
        return this
    }

    pub fn to_be_bytes(&self)->[u8; 0xE]{
        [
            self.unk_0.into_iter().flat_map(i16::to_be_bytes).collect::<Vec<_>>().as_slice(),
            self.unk_6.into_iter().flat_map(i16::to_be_bytes).collect::<Vec<_>>().as_slice(),
            &[self.unk_c, 0]
        ].concat().try_into().unwrap()
    }
}