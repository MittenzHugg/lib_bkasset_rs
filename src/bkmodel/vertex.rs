use std::cmp;
use std::ops::{Deref, DerefMut};

use libultra::Vtx;


#[derive(Debug)]
pub struct BKVertexListHeader{
    min : [i16; 3],
    max : [i16; 3],
    center: [i16; 3],
    local_norm: i16,
    len: i16,
    global_norm: i16,
}

impl BKVertexListHeader{
    pub fn from_be_bytes(bytes: [u8; 0x18]) -> BKVertexListHeader{
        let mut shorts : Vec<i16> = bytes
            .chunks_exact(2)
            .map(|bytes| i16::from_be_bytes(bytes.try_into().unwrap()))
            .collect();


        let this = BKVertexListHeader{
            min :         shorts[0..3].try_into().unwrap(),
            max :         shorts[3..6].try_into().unwrap(),
            center :      shorts[6..9].try_into().unwrap(),
            local_norm :  shorts[9],
            len :         shorts[10],
            global_norm : shorts[11],
        };
        
        #[cfg(feature = "test_byte_matching")]assert_eq!(bytes, this.to_be_bytes(), "\n{:#?}", this);
        return this

    }

    pub fn to_be_bytes(&self)->[u8; 0x18]{
        [
            self.min.into_iter().flat_map(i16::to_be_bytes).collect::<Vec<_>>().as_slice(),
            self.max.into_iter().flat_map(i16::to_be_bytes).collect::<Vec<_>>().as_slice(),
            self.center.into_iter().flat_map(i16::to_be_bytes).collect::<Vec<_>>().as_slice(),
            self.local_norm.to_be_bytes().as_slice(),
            self.len.to_be_bytes().as_slice(),
            self.global_norm.to_be_bytes().as_slice(),
        ].concat().try_into().unwrap()
    }
}

#[derive(Debug)]
pub struct BKVertexList{
    pub vertex : Vec<Vtx>,
    preserved_global_norm : Option<i16>,
}

impl BKVertexList{
    pub fn from_be_bytes(bytes: &[u8]) -> BKVertexList{
        let header = BKVertexListHeader::from_be_bytes(bytes[0..0x18].try_into().unwrap());
        let vertex = bytes[0x18..]
            .chunks_exact(0x10)
            .take(header.len as usize)
            .map(|bytes| Vtx::from_be_bytes(bytes))
            .collect();
        let mut this = BKVertexList{
            vertex, preserved_global_norm: None,
        };
        
        if this.global_norm() != header.global_norm {
            this.preserved_global_norm = Some(header.global_norm);
        };

        #[cfg(feature = "test_byte_matching")]assert_eq!(bytes[0..0x18 + 0x10*header.len as usize], this.to_be_bytes(), "\n{:#?}", this);
        return this 
    }

    pub fn to_be_bytes(&self)->Vec<u8>{
        let mut bytes = Vec::with_capacity(self.size());
        bytes = [
            self.get_header().to_be_bytes().as_slice(),
            self.vertex.iter().flat_map(Vtx::to_bytes).collect::<Vec<_>>().as_slice()
        ].concat();
        return bytes
    }

    pub fn min(&self) -> [i16; 3]{
        return self.vertex.iter()
            .map(|v| v.ob)
            .reduce(|a, b| [cmp::min(a[0], b[0]), cmp::min(a[1], b[1]), cmp::min(a[2], b[2])])
            .unwrap_or([0;3])
    }

    pub fn max(&self) -> [i16; 3]{
        return self.vertex.iter()
            .map(|v| v.ob)
            .reduce(|a, b| [cmp::max(a[0], b[0]), cmp::max(a[1], b[1]), cmp::max(a[2], b[2])])
            .unwrap_or([0;3])
    }

    pub fn center(&self) -> [i16; 3]{
        let max = self.max();
        let min = self.min();

        return [
            ((max[0] as i32 + min[0] as i32)/2) as i16, 
            ((max[1] as i32 + min[1] as i32)/2) as i16, 
            ((max[2] as i32 + min[2] as i32)/2) as i16
        ];
    }

    pub fn local_norm(&self) -> i16{
        let center = self.center();
        return match self.vertex.iter()
        .map(|v| [(v.ob[0] as f64 - center[0] as f64), (v.ob[1] as f64 - center[1] as f64), (v.ob[2] as f64 - center[2] as f64)])
        .map(|d|  d[0]*d[0] + d[1]*d[1] + d[2]*d[2])
        .reduce(|a, b| f64::max(a, b)) {
            None => 0,
            Some(val) => val.sqrt() as i16,
        };
    }

    pub fn global_norm(&self) -> i16{
        return match self.vertex.iter()
        .map(|v| [v.ob[0] as f64, v.ob[1] as f64, v.ob[2] as f64])
        .map(|d|  d[0]*d[0] + d[1]*d[1] + d[2]*d[2])
        .reduce(|a, b| f64::max(a, b)) {
            None => 0,
            Some(val) => val.sqrt() as i16,
        };
    }

    pub fn get_header(&self) -> BKVertexListHeader{
        BKVertexListHeader{
            min:    self.min(),
            max:    self.max(),
            center: self.center(),
            local_norm: self.local_norm(),
            len:    self.len() as i16,
            global_norm: self.preserved_global_norm.unwrap_or(self.global_norm()),
        }
    }

    pub fn size(&self) -> usize{
        0x18 + 0x10*self.len()
    }
}

impl Deref for BKVertexList{
    type Target = Vec<Vtx>;
    fn deref(&self) -> &Vec<Vtx> { &self.vertex }
}

impl DerefMut for BKVertexList {
    fn deref_mut(&mut self) -> &mut Vec<Vtx> { &mut self.vertex }
}