pub struct BKModelUnk14ListHeader{
    type_0_count: usize,
    type_1_count: usize,
    type_2_count: usize,
    unk_6: i16,
}

impl BKModelUnk14ListHeader {
    pub fn from_be_bytes(bytes: [u8; 0x8])->Self{
        BKModelUnk14ListHeader { 
            type_0_count: u16::from_be_bytes([bytes[0], bytes[1]]) as usize, 
            type_1_count: u16::from_be_bytes([bytes[2], bytes[3]]) as usize,
            type_2_count: u16::from_be_bytes([bytes[4], bytes[5]]) as usize,
            unk_6:  i16::from_be_bytes([bytes[6], bytes[7]])
        }
    }

    pub fn to_be_bytes(&self)->Vec<u8>{
        vec![
            (self.type_0_count as u16).to_be_bytes(),
            (self.type_1_count as u16).to_be_bytes(),
            (self.type_2_count as u16).to_be_bytes(),
            (self.unk_6 as u16).to_be_bytes(),
        ].concat()
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
    pub fn from_be_bytes(bytes: &[u8])->Option<Self>{
        let header_bytes: [u8; 8] = bytes[..8].try_into().ok()?;
        let bytes = &bytes[8..];
        let header = BKModelUnk14ListHeader::from_be_bytes(header_bytes);
        
        let type_0_list: Vec<BKModelUnk14Type0> = bytes.chunks_exact(0x18)
            .map(|bytes| BKModelUnk14Type0::from_be_bytes(bytes.try_into().unwrap()))
            .take(header.type_0_count)
            .collect();
        let bytes = &bytes[0x18*header.type_0_count..];

        let type_1_list: Vec<BKModelUnk14Type1> = bytes.chunks_exact(0x10)
            .map(|bytes| BKModelUnk14Type1::from_be_bytes(bytes.try_into().unwrap()))
            .take(header.type_1_count)
            .collect();
        let bytes = &bytes[0x10*header.type_1_count..];

        let type_2_list: Vec<BKModelUnk14Type2> = bytes.chunks_exact(0xC)
                .map(|bytes| BKModelUnk14Type2::from_be_bytes(bytes.try_into().unwrap()))
                .take(header.type_2_count)
                .collect();

        return Some(BKModelUnk14List {  
            scale : header.unk_6,
            type_0_list,
            type_1_list,
            type_2_list,
        })
    }

    pub fn to_be_bytes(&self)->Vec<u8>{
        vec![
            self.get_header().to_be_bytes(),
            self.type_0_list.iter().map(|a| a.to_be_bytes()).flatten().collect(),
            self.type_1_list.iter().map(|b| b.to_be_bytes()).flatten().collect(),
            self.type_2_list.iter().map(|c| c.to_be_bytes()).flatten().collect(),
        ].concat()
    }

    pub fn get_header(&self)->BKModelUnk14ListHeader{
        BKModelUnk14ListHeader{
            type_0_count: self.type_0_list.len(),
            type_1_count: self.type_1_list.len(),
            type_2_count: self.type_2_list.len(),
            unk_6: self.scale,
        }
    }

    pub fn byte_size(&self)->usize{
        0x8 + self.type_0_list.len()*0x18 + self.type_1_list.len()*0x10 + self.type_2_list.len()*0xC
    }
}


#[derive(Debug)]
pub struct BKModelUnk14Type0{
    pub unk_0:  [i16; 3],
    pub unk_6:  [i16; 3],
    pub unk_c:  [i16; 3],
    pub unk_12: [u8; 3],
    pub unk_15: u8,
    pub unk_16: i8,
    pub pad_17: [u8; 1]
}

impl BKModelUnk14Type0 {
    pub fn from_be_bytes(bytes: [u8; 0x18])->Self{
        let mut short_iter = bytes.chunks_exact(2).map(|bytes| i16::from_be_bytes(bytes.try_into().unwrap()));

        BKModelUnk14Type0{
            unk_0: [short_iter.next().unwrap().clone(), short_iter.next().unwrap().clone(), short_iter.next().unwrap().clone()],
            unk_6: [short_iter.next().unwrap().clone(), short_iter.next().unwrap().clone(), short_iter.next().unwrap().clone()],
            unk_c: [short_iter.next().unwrap().clone(), short_iter.next().unwrap().clone(), short_iter.next().unwrap().clone()],
            unk_12: [bytes[0x12], bytes[0x13], bytes[0x14]],
            unk_15: bytes[0x15],
            unk_16: bytes[0x16] as i8,
            pad_17: [bytes[0x17]],
        }
    }

    pub fn to_be_bytes(&self)->Vec<u8>{
        vec![
            self.unk_0.iter().map(|x| x.to_be_bytes()).flatten().collect(),
            self.unk_6.iter().map(|x| x.to_be_bytes()).flatten().collect(),
            self.unk_c.iter().map(|x| x.to_be_bytes()).flatten().collect(),
            self.unk_12.to_vec(),
            vec![self.unk_15, self.unk_16 as u8, self.pad_17[0]]
        ].concat()
    }
}

#[derive(Debug)]
pub struct BKModelUnk14Type1{
    pub raw_bytes: [u8; 0x10],
}

impl BKModelUnk14Type1 {
    pub fn from_be_bytes(bytes: [u8; 0x10])->Self{
        BKModelUnk14Type1{
            raw_bytes: bytes
        }
    }

    pub fn to_be_bytes(&self)->Vec<u8>{
        self.raw_bytes.to_vec()
    }
}

#[derive(Debug)]
pub struct BKModelUnk14Type2{
    pub raw_bytes: [u8; 0xC],
}

impl BKModelUnk14Type2 {
    pub fn from_be_bytes(bytes: [u8; 0xC])->Self{
        BKModelUnk14Type2{
            raw_bytes: bytes
        }
    }

    pub fn to_be_bytes(&self)->Vec<u8>{
        self.raw_bytes.to_vec()
    }
}
