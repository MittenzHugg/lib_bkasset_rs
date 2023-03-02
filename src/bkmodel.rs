
use libultra::{Vtx, F3dex};
use std::cmp;
use std::ops::{Deref, DerefMut};
use std::fmt;

trait BKGeo : std::fmt::Debug{
}

impl dyn BKGeo{
    fn from_be_bytes(in_bytes: &[u8])->Box<dyn BKGeo>{
        let cmd = u32::from_be_bytes(in_bytes[0..4].try_into().unwrap());
        match cmd {
            0 => Box::new(GeoCmd0::from_be_bytes(in_bytes)),
            2 => Box::new(GeoCmd2::from_be_bytes(in_bytes)),
            3 => Box::new(GeoCmd3::from_be_bytes(in_bytes)),
            5 => Box::new(GeoCmd5::from_be_bytes(in_bytes)),
            10 => Box::new(GeoCmdA::from_be_bytes(in_bytes)),
            12 => Box::new(GeoCmdC{}),
            15 => Box::new(GeoCmdF{}),
            _ => panic!("Unknown gfx_command {}", cmd),
        }
    }
}

#[derive(Debug)]
struct GeoCmdC{}
impl BKGeo for GeoCmdC{}
#[derive(Debug)]
struct GeoCmdF{}
impl BKGeo for GeoCmdF{}


#[derive(Debug)]
struct GeoCmd0 {
    children: Option<BKGeoList>,
    position: [f32; 3],
}

impl GeoCmd0 {
    fn from_be_bytes(in_bytes: &[u8])->Self{
        let cmd_0  = i32::from_be_bytes(in_bytes[0..4].try_into().unwrap());
        assert_eq!(cmd_0, 0);
        let child_offset = i16::from_be_bytes(in_bytes[8..10].try_into().unwrap()) as usize;
        GeoCmd0{
            children : if child_offset == 0 {None} else {Some(BKGeoList::from_be_bytes(&in_bytes[child_offset..]))},
            position : [f32::from_be_bytes(in_bytes[0xC..0x10].try_into().unwrap()), f32::from_be_bytes(in_bytes[0x10..0x14].try_into().unwrap()), f32::from_be_bytes(in_bytes[0x14..0x18].try_into().unwrap())],
        }
    }
}

impl BKGeo for GeoCmd0{}

#[derive(Debug)]
struct GeoCmd2 {
    children: Option<BKGeoList>,
    mtx_index: usize,
}

impl GeoCmd2 {
    fn from_be_bytes(in_bytes: &[u8])->Self{
        let cmd_0  = i32::from_be_bytes(in_bytes[0..4].try_into().unwrap());
        assert_eq!(cmd_0, 2);
        let child_offset = in_bytes[8] as usize;
        GeoCmd2{
            children : if child_offset == 0 {None} else {Some(BKGeoList::from_be_bytes(&in_bytes[child_offset..]))},
            mtx_index :in_bytes[9] as usize,
        }
    }
}

impl BKGeo for GeoCmd2{}

#[derive(Debug)]
struct GeoCmd3 {
    gfx_offset: usize,
}

impl GeoCmd3 {
    fn from_be_bytes(in_bytes: &[u8])->Self{
        let cmd_0  = i32::from_be_bytes(in_bytes[0..4].try_into().unwrap());
        assert_eq!(cmd_0, 3);
        GeoCmd3{
            gfx_offset : i16::from_be_bytes(in_bytes[8..10].try_into().unwrap()) as usize
        }
    }
}

impl BKGeo for GeoCmd3{}

#[derive(Debug)]
struct GeoCmd5{
    gfx_index_list: Vec<usize>,
}

impl GeoCmd5 {
    fn from_be_bytes(in_bytes: &[u8])->Self{
        let cmd_0  = i32::from_be_bytes(in_bytes[0..4].try_into().unwrap());
        assert_eq!(cmd_0, 5);
        let mut offset_iter = in_bytes[8..].chunks_exact(2).map(|b| i16::from_be_bytes([b[0], b[1]]) as usize);
        let gfx_index_list = offset_iter.clone().take(1)
            .chain(offset_iter.skip(1).take_while(|&val| val != 0))
            .collect();
        let child_offset = i16::from_be_bytes(in_bytes[8..10].try_into().unwrap()) as usize;
        GeoCmd5{
            gfx_index_list,
        }
    }
}

impl BKGeo for GeoCmd5{}

#[derive(Debug)]
struct GeoCmdA{
    unk_8: usize,
    unk_a: usize,
    unk_c: [f32; 3],
}

impl GeoCmdA {
    fn from_be_bytes(in_bytes: &[u8])->Self{
        let cmd_0  = i32::from_be_bytes(in_bytes[0..4].try_into().unwrap());
        assert_eq!(cmd_0, 10);
        let unk_8 = i16::from_be_bytes([in_bytes[8], in_bytes[9]]) as usize;
        let unk_a = i16::from_be_bytes([in_bytes[0xa], in_bytes[0xb]]) as usize;
        let unk_c = [f32::from_be_bytes(in_bytes[0xC..0x10].try_into().unwrap()), f32::from_be_bytes(in_bytes[0x10..0x14].try_into().unwrap()), f32::from_be_bytes(in_bytes[0x14..0x18].try_into().unwrap())];
        GeoCmdA{ unk_8, unk_a, unk_c}
    }
}

impl BKGeo for GeoCmdA{}

#[derive(Debug)]
pub struct BKGeoList{
    geo: Vec<Box<dyn BKGeo>>
}

impl BKGeoList{
    fn from_be_bytes(in_bytes: &[u8])->Self{
        let mut root_offsets : Vec<usize> = vec![0];
        loop {
            let i = root_offsets.last().unwrap();
            
            let size = i32::from_be_bytes(in_bytes[i+4..i+8].try_into().unwrap()) as usize;
            if size != 0 {
                root_offsets.push(i + size);
            }
            else{
                break;
            }
        }
        BKGeoList{geo: root_offsets.iter().map(|&offset| <dyn BKGeo>::from_be_bytes(&in_bytes[offset..])).collect()}
    }
}

use super::bktexture::*;

#[derive(Debug)]
pub struct BKTextureList{
    pub texture_headers : Vec<BKTextureHeader>,
}

impl BKTextureList{
    pub fn from_be_bytes(in_bytes: &[u8])->BKTextureList{
        let _byte_count = u32::from_be_bytes(in_bytes[0..4].try_into().unwrap()) as usize;
        let count = u16::from_be_bytes(in_bytes[4..6].try_into().unwrap()) as usize;
        let texture_headers :Vec<_>= in_bytes[8..].chunks_exact(0x10).take(count)
            .map(|bytes| BKTextureHeader::from_be_bytes(bytes))
            .collect();
        BKTextureList{texture_headers}
    }
}

pub struct BKGfxList{
    gfx:Vec<F3dex>,
}

impl BKGfxList{
    pub fn from_be_bytes(in_bytes: &[u8])->BKGfxList{
        let count = u32::from_be_bytes(in_bytes[0..4].try_into().unwrap()) as usize;
        let gfx = in_bytes[8..].chunks_exact(8)
            .map(|b| F3dex::from(u64::from_be_bytes(b.try_into().unwrap())))
            .take(count)
            .collect();
        BKGfxList{gfx}
    }

    pub fn to_bytes(&self)->Vec<u8>{
        let len = self.len() as u32;
        let mut out : Vec<u8> = len.to_be_bytes().to_vec();
        out.append(&mut vec![0;4]);
        out.append(&mut self.gfx.iter()
            .map(|&g| u64::from(g).to_be_bytes())
            .flatten()
            .collect::<Vec<u8>>()
        );
        return out;
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
    pub fn from_be_bytes(in_bytes: &[u8]) -> BKVertexListHeader{
        let mut short_iter = in_bytes
            .chunks_exact(2)
            .map(|bytes| i16::from_be_bytes(bytes.try_into().unwrap()));
        let min = [short_iter.next().unwrap().clone(), short_iter.next().unwrap().clone(), short_iter.next().unwrap().clone()];
        let max = [short_iter.next().unwrap().clone(), short_iter.next().unwrap().clone(), short_iter.next().unwrap().clone()];
        let center = [short_iter.next().unwrap().clone(), short_iter.next().unwrap().clone(), short_iter.next().unwrap().clone()];
        let local_norm = short_iter.next().unwrap().clone();
        let len = short_iter.next().unwrap().clone();
        let global_norm = short_iter.next().unwrap().clone();

        return BKVertexListHeader{
            min,
            max,
            center,
            local_norm,
            len,
            global_norm,
        }
    }
}

#[derive(Debug)]
pub struct BKVertexList{
    pub vertex : Vec<Vtx>,
}

impl BKVertexList{
    pub fn from_be_bytes(in_bytes: &[u8]) -> BKVertexList{
        let header = BKVertexListHeader::from_be_bytes(in_bytes);
        let vertex = in_bytes[0x18..]
            .chunks_exact(0x10)
            .take(header.len as usize)
            .map(|bytes| Vtx::from_be_bytes(bytes))
            .collect();
        return BKVertexList{vertex}
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

        return [(max[0] - min[0])/2, (max[1] - min[1])/2, (max[2] - min[2])/2];
    }

    pub fn local_norm(&self) -> i16{
        let center = self.center();
        return match self.vertex.iter()
        .map(|v| [(v.ob[0] - center[0]) as f64, (v.ob[1] - center[1]) as f64, (v.ob[2] - center[2]) as f64])
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
            global_norm: self.local_norm(),

        }
    }
}

impl Deref for BKVertexList{
    type Target = Vec<Vtx>;
    fn deref(&self) -> &Vec<Vtx> { &self.vertex }
}

impl DerefMut for BKVertexList {
    fn deref_mut(&mut self) -> &mut Vec<Vtx> { &mut self.vertex }
}

// #[derive(Debug)]
pub struct BKCollisionTri{
    vtx: [i16; 3],
    unk_6: i16,
    flags: u32,
}

impl BKCollisionTri{
    pub fn from_be_bytes(in_bytes: &[u8]) -> BKCollisionTri{
        let vtx = [
            i16::from_be_bytes([in_bytes[0], in_bytes[1]]), 
            i16::from_be_bytes([in_bytes[2], in_bytes[3]]),
            i16::from_be_bytes([in_bytes[4], in_bytes[5]])
        ];
        let unk_6 = i16::from_be_bytes([in_bytes[6], in_bytes[7]]);
        let flags = u32::from_be_bytes(in_bytes[8..0xC].try_into().unwrap());
        BKCollisionTri{vtx, unk_6, flags}
    }
}
impl fmt::Debug for BKCollisionTri{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BKCollisionTri: {{vtx: {:>4?}, unk_16: 0x{:03X}, flags: 0x{:03X}}}", self.vtx, self.unk_6, self.flags)
    }
}

pub struct BKCollisionMesh{
    tri_start: i16,
    size: i16,
}

impl BKCollisionMesh{
    pub fn from_be_bytes(in_bytes: &[u8]) -> BKCollisionMesh{
        let tri_start = i16::from_be_bytes([in_bytes[0], in_bytes[1]]);
        let size = i16::from_be_bytes([in_bytes[2], in_bytes[3]]);
        BKCollisionMesh{tri_start, size}
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
    geo_cnt: i16, //geo_cnt
    unk_12: i16, //scale
    tri_cnt: i16, //tri_cnt
    geo: Vec<BKCollisionMesh>,
    tri: Vec<BKCollisionTri>,
}

impl BKCollisionList{
    pub fn from_be_bytes(in_bytes: &[u8]) -> BKCollisionList{
        let mut short_iter = in_bytes
            .chunks_exact(2)
            .map(|bytes| i16::from_be_bytes(bytes.try_into().unwrap()));
        let unk_0 = [short_iter.next().unwrap().clone(), short_iter.next().unwrap().clone(), short_iter.next().unwrap().clone()];
        let unk_6 = [short_iter.next().unwrap().clone(), short_iter.next().unwrap().clone(), short_iter.next().unwrap().clone()];
        let unk_c = short_iter.next().unwrap().clone();
        let unk_e = short_iter.next().unwrap().clone();
        let geo_cnt = short_iter.next().unwrap().clone();
        let unk_12 = short_iter.next().unwrap().clone();
        let tri_cnt = short_iter.next().unwrap().clone();
        let geo = in_bytes[0x18..].chunks_exact(4)
            .take(geo_cnt as usize)
            .map(|bytes| BKCollisionMesh::from_be_bytes(bytes))
            .collect();
        let tri = in_bytes[0x18 + (geo_cnt as usize)*0x4 .. ].chunks_exact(0xC)
            .take(tri_cnt as usize)
            .map(|bytes| BKCollisionTri::from_be_bytes(bytes))
            .collect();

        return BKCollisionList{
            unk_0,
            unk_6,
            unk_c,
            unk_e,
            geo_cnt,
            unk_12,
            tri_cnt,
            geo,
            tri,
        }
    }
}

#[derive(Debug)]
pub struct BKModelHeader{
    pub geo_list_offset : usize,
    pub texture_list_offset: usize,
    pub vtx_list_offset : usize,
    pub gfx_list_offset : usize,
    pub collision_list_offset : usize,
}

impl BKModelHeader{
    pub fn from_be_bytes(in_bytes: &[u8]) -> BKModelHeader{
        let geo_list_offset       = i32::from_be_bytes(in_bytes[0x4..0x8].try_into().unwrap()) as usize;
        let texture_list_offset   = u16::from_be_bytes(in_bytes[0x8..0xA].try_into().unwrap()) as usize;
        let gfx_list_offset       = i32::from_be_bytes(in_bytes[0xC..0x10].try_into().unwrap()) as usize;
        let vtx_list_offset       = i32::from_be_bytes(in_bytes[0x10..0x14].try_into().unwrap()) as usize;
        let collision_list_offset = i32::from_be_bytes(in_bytes[0x1C..0x20].try_into().unwrap()) as usize;
        return BKModelHeader{
            geo_list_offset, 
            texture_list_offset, 
            gfx_list_offset, 
            vtx_list_offset, 
            collision_list_offset
        }
    }
}

#[derive(Debug)]
pub struct BKModel {
    pub geo_list      : Option<BKGeoList>,
    pub texture_list  : Option<BKTextureList>,
    pub display_list  : Option<BKGfxList>, 
    pub vertices      : Option<BKVertexList>,
    pub collision_list: Option<BKCollisionList>,
}

impl BKModel {
    pub fn from_be_bytes(in_bytes: &[u8]) -> BKModel{

        //get header from bytes
        let header = BKModelHeader::from_be_bytes(in_bytes);
        println!("{:X?}", header);
        //generate model parts
        let geo_list = if header.geo_list_offset == 0 {None} else {Some(BKGeoList::from_be_bytes(&in_bytes[header.geo_list_offset..]))};
        let texture_list  = if header.texture_list_offset == 0 {None} else {Some(BKTextureList::from_be_bytes(&in_bytes[header.texture_list_offset..]))};
        let vertices = if header.vtx_list_offset == 0 {None} else {Some(BKVertexList::from_be_bytes(&in_bytes[header.vtx_list_offset..]))};
        let gfx = if header.vtx_list_offset == 0 {None} else {Some(BKGfxList::from_be_bytes(&in_bytes[header.gfx_list_offset..]))};
        let collision_list = if header.collision_list_offset == 0 {None} else {Some(BKCollisionList::from_be_bytes(&in_bytes[header.collision_list_offset..]))};
        return BKModel{geo_list, texture_list, display_list: gfx, vertices, collision_list}
    }
}
