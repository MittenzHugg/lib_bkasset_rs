use std::{fmt, thread::current, process::Output};

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

mod texture;
pub use texture::{*};

mod display_list;
pub use display_list::{*};

mod vertex;
pub use vertex::{*};

mod unk_14;
pub use unk_14::{*};

mod animation;
pub use animation::{*};

mod collision;
pub use collision::{*};

mod unk_20;
pub use unk_20::{*};

mod meshes;
pub use meshes::{*};

mod unk_28;
pub use unk_28::{*};

mod animated_texture;
pub use animated_texture::BKAnimatedTextureList;

#[derive(Debug, PartialEq)]
pub struct BKModelHeader{
    pub geo_list_offset : usize,
    pub texture_list_offset: usize,
    pub unk_a: u16,
    pub gfx_list_offset : usize,
    pub vtx_list_offset : usize,
    pub unk_14_offset : usize,
    pub unk_18_offset : usize,
    pub collision_list_offset : usize,
    pub unk_20_offset : usize,
    pub unk_24_offset : usize,
    pub unk_28_offset : usize,
    pub unk_2c_offset : usize,
    pub unk_30: u16,
    pub unk_32: u16,
    pub unk_34: f32,
}

impl BKModelHeader{
    pub fn from_be_bytes(in_bytes: &[u8; 0x38]) -> BKModelHeader{
        assert_eq!(in_bytes[..4], [0,0,0,0xB]);
        return BKModelHeader{
            geo_list_offset       : u32::from_be_bytes(in_bytes[0x4..0x8].try_into().unwrap()) as usize,
            texture_list_offset   : u16::from_be_bytes(in_bytes[0x8..0xA].try_into().unwrap()) as usize,
            unk_a                 : u16::from_be_bytes(in_bytes[0xA..0xC].try_into().unwrap()),
            gfx_list_offset       : u32::from_be_bytes(in_bytes[0xC..0x10].try_into().unwrap()) as usize,
            vtx_list_offset       : u32::from_be_bytes(in_bytes[0x10..0x14].try_into().unwrap()) as usize,
            unk_14_offset         : u32::from_be_bytes(in_bytes[0x14..0x18].try_into().unwrap()) as usize,
            unk_18_offset         : u32::from_be_bytes(in_bytes[0x18..0x1C].try_into().unwrap()) as usize,
            collision_list_offset : u32::from_be_bytes(in_bytes[0x1C..0x20].try_into().unwrap()) as usize,
            unk_20_offset         : u32::from_be_bytes(in_bytes[0x20..0x24].try_into().unwrap()) as usize,
            unk_24_offset         : u32::from_be_bytes(in_bytes[0x24..0x28].try_into().unwrap()) as usize,
            unk_28_offset         : u32::from_be_bytes(in_bytes[0x28..0x2C].try_into().unwrap()) as usize,
            unk_2c_offset         : u32::from_be_bytes(in_bytes[0x2C..0x30].try_into().unwrap()) as usize,
            unk_30                : u16::from_be_bytes(in_bytes[0x30..0x32].try_into().unwrap()),
            unk_32                : u16::from_be_bytes(in_bytes[0x32..0x34].try_into().unwrap()),
            unk_34                : f32::from_be_bytes(in_bytes[0x34..0x38].try_into().unwrap()),
        }
    }

    pub fn to_be_bytes(&self) -> [u8; 0x38]{
        [
            [0,0,0,0xB].as_slice(),
            (self.geo_list_offset as u32).to_be_bytes().as_slice(),
            (self.texture_list_offset as u16).to_be_bytes().as_slice(),
            self.unk_a.to_be_bytes().as_slice(),
            (self.gfx_list_offset as u32).to_be_bytes().as_slice(),
            (self.vtx_list_offset as u32).to_be_bytes().as_slice(),
            (self.unk_14_offset as u32).to_be_bytes().as_slice(),
            (self.unk_18_offset as u32).to_be_bytes().as_slice(),
            (self.collision_list_offset as u32).to_be_bytes().as_slice(),
            (self.unk_20_offset as u32).to_be_bytes().as_slice(),
            (self.unk_24_offset as u32).to_be_bytes().as_slice(),
            (self.unk_28_offset as u32).to_be_bytes().as_slice(),
            (self.unk_2c_offset as u32).to_be_bytes().as_slice(),
            self.unk_30.to_be_bytes().as_slice(),
            self.unk_32.to_be_bytes().as_slice(),
            self.unk_34.to_be_bytes().as_slice(),
        ].concat().try_into().unwrap()
    }
}




#[derive(Debug)]
pub struct BKModel {
    pub header : BKModelHeader,
    pub texture_list  : Option<BKTextureList>,
    pub display_list  : Option<BKGfxList>, 
    pub vertices      : Option<BKVertexList>,
    pub unk_14_list   : Option<BKModelUnk14List>,
    pub animation_list: Option<BKAnimationList>,
    pub collision_list: Option<BKCollisionList>,
    pub unk_20_list   : Option<BKModelUnk20List>,
    pub mesh_list     : Option<BKMeshList>,
    pub unk_28_list   : Option<BKModelUnk28List>,
    pub animated_texture_list   : Option<BKAnimatedTextureList>,
    // pub geo_list      : Option<BKGeoList>,
    pub data : Vec<u8>,
}

impl BKModel {
    pub fn try_from_be_bytes(in_bytes: &[u8]) -> Option<BKModel>{
        if in_bytes[0..4] != [0,0,0,0xB] {return None;}
        
        let header = BKModelHeader::from_be_bytes(in_bytes[0..0x38].try_into().unwrap());
        let texture_list  = if header.texture_list_offset == 0 {None} else {Some(BKTextureList::from_be_bytes(&in_bytes[header.texture_list_offset..]))};
        let gfx = if header.vtx_list_offset == 0 {None} else {BKGfxList::try_from_be_bytes(&in_bytes[header.gfx_list_offset..]).ok()};
        let vertices = if header.vtx_list_offset == 0 {None} else {Some(BKVertexList::from_be_bytes(&in_bytes[header.vtx_list_offset..]))};
        let unk_14_list = if header.unk_14_offset == 0 {None} else {BKModelUnk14List::try_from_be_bytes(&in_bytes[header.unk_14_offset..]).ok()};
        let animation_list = if header.unk_18_offset == 0 {None} else {BKAnimationList::try_from_be_bytes(&in_bytes[header.unk_18_offset..]).ok()};
        let collision_list = if header.collision_list_offset == 0 {None} else {BKCollisionList::try_from_be_bytes(&in_bytes[header.collision_list_offset..]).ok()};
        let unk_20_list = if header.unk_20_offset == 0 {None} else {BKModelUnk20List::try_from_be_bytes(&in_bytes[header.unk_20_offset..]).ok()};
        let mesh_list = if header.unk_24_offset == 0 {None} else {BKMeshList::try_from_be_bytes(&in_bytes[header.unk_24_offset..]).ok()};
        let unk_28_list = if header.unk_28_offset == 0 {None} else {BKModelUnk28List::try_from_be_bytes(&in_bytes[header.unk_28_offset..]).ok()};
        let animated_texture_list = if header.unk_2c_offset == 0 {None} else {Some(BKAnimatedTextureList::from_be_bytes(in_bytes[header.unk_2c_offset..header.unk_2c_offset+0x20].try_into().unwrap()))};
        
        let offset : usize = [
            Some(0x38),
            texture_list.as_ref().map(BKTextureList::size),
            gfx.as_ref().map(BKGfxList::size),
            vertices.as_ref().map(BKVertexList::size),
            unk_14_list.as_ref().map(BKModelUnk14List::size),
            collision_list.as_ref().map(BKCollisionList::size),
            mesh_list.as_ref().map(BKMeshList::size),
            unk_20_list.as_ref().map(BKModelUnk20List::size),
            unk_28_list.as_ref().map(BKModelUnk28List::size),
            animation_list.as_ref().map(BKAnimationList::size),
            animated_texture_list.as_ref().map(BKAnimatedTextureList::size),
        ].iter().flat_map(|size| size).sum();

        Some(BKModel { 
            header,
            texture_list,
            display_list: gfx,
            vertices,
            unk_14_list,
            animation_list,
            collision_list,
            unk_20_list,
            mesh_list,
            unk_28_list,
            animated_texture_list,
            data: in_bytes[offset..].to_vec() 
        })
    }
    // pub fn from_be_bytes(in_bytes: &[u8]) -> BKModel{

    //     //get header from bytes
    //     let header = BKModelHeader::from_be_bytes(in_bytes);
    //     println!("{:X?}", header);
    //     //generate model parts
    //     let geo_list = if header.geo_list_offset == 0 {None} else {Some(BKGeoList::from_be_bytes(&in_bytes[header.geo_list_offset..]))};
    //     let collision_list = if header.collision_list_offset == 0 {None} else {Some(BKCollisionList::from_be_bytes(&in_bytes[header.collision_list_offset..]))};
    //     return BKModel{geo_list, texture_list, display_list: gfx, vertices, collision_list}
    // }

    pub fn to_be_bytes(&self) -> Vec<u8> {
            /*packing order*/
            //texture
            //display_list
            //vtx_list,
            //unk14
            //collision,
            //unk24
            //unk20
            //unk28
            //animation_list
            //animated_textures
            //geo_list
        [  
            Some(self.create_header().to_be_bytes().to_vec()), 
            self.texture_list.as_ref().map(BKTextureList::to_be_bytes),
            self.display_list.as_ref().map(BKGfxList::to_be_bytes),
            self.vertices.as_ref().map(BKVertexList::to_be_bytes),
            self.unk_14_list.as_ref().map(BKModelUnk14List::to_be_bytes),
            self.collision_list.as_ref().map(BKCollisionList::to_be_bytes),
            self.mesh_list.as_ref().map(BKMeshList::to_be_bytes),
            self.unk_20_list.as_ref().map(BKModelUnk20List::to_be_bytes),
            self.unk_28_list.as_ref().map(BKModelUnk28List::to_be_bytes),
            self.animation_list.as_ref().map(BKAnimationList::to_be_bytes),
            self.animated_texture_list.as_ref().map(|x| x.to_be_bytes().to_vec()),
            Some(self.data.clone())
        ].into_iter().flatten().flatten().collect::<Vec<u8>>()
    }

    pub fn create_header(&self)->BKModelHeader {
            let current_offset = 0x38;

            let (texture_list_offset, current_offset) = match &self.texture_list {
                Some(texture_list) => (current_offset, current_offset + texture_list.size()),
                None => (0, current_offset)
            };

            let (gfx_list_offset, current_offset) = match &self.display_list {
                Some(gfx_list) => (current_offset, current_offset + gfx_list.size()),
                None => (0, current_offset)
            };

            let (vtx_list_offset, current_offset) = match &self.vertices {
                Some(vtx_list) => (current_offset, current_offset + vtx_list.size()),
                None => (0, current_offset)
            };

            let (unk_14_offset, current_offset) = match &self.unk_14_list {
                Some(unk14_list) => (current_offset, current_offset + unk14_list.size()),
                None => (0, current_offset)
            };

            let (collision_list_offset, current_offset) = match &self.collision_list {
                Some(collision_list) => (current_offset, current_offset + collision_list.size()),
                None => (0, current_offset)
            };

            let (mesh_list_offset, current_offset) = match &self.mesh_list {
                Some(mesh_list) => (current_offset, current_offset + mesh_list.size()),
                None => (0, current_offset)
            };

            let (unk_20_offset, current_offset) = match &self.unk_20_list {
                Some(unk_20_list) => (current_offset, current_offset + unk_20_list.size()),
                None => (0, current_offset)
            };

            let (unk_28_offset, current_offset) = match &self.unk_28_list {
                Some(unk_28_list) => (current_offset, current_offset + unk_28_list.size()),
                None => (0, current_offset)
            };


            let (animation_offset, current_offset) = match &self.animation_list {
                Some(anim_list) => (current_offset, current_offset + anim_list.size()),
                None => (0, current_offset)
            };

            let (anim_texture_offset, current_offset) = match &self.animated_texture_list {
                Some(anim_txtr_list) => (current_offset, current_offset + anim_txtr_list.size()),
                None => (0, current_offset)
            };

            let geo_list_offset = current_offset;

            BKModelHeader { 
                geo_list_offset, 
                texture_list_offset, 
                unk_a: self.header.unk_a, 
                gfx_list_offset, 
                vtx_list_offset, 
                unk_14_offset, 
                unk_18_offset: animation_offset, 
                collision_list_offset, 
                unk_20_offset, 
                unk_24_offset: mesh_list_offset, 
                unk_28_offset, 
                unk_2c_offset: anim_texture_offset, 
                unk_30: self.header.unk_30, 
                unk_32: self.header.unk_32,
                unk_34: self.header.unk_34
            }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use std::fs;
    
    #[test]
    fn byte_matching(){
        let model_file_paths = fs::read_dir("assets/model/").unwrap();
        let mut nonmatches: Vec<String> = Vec::new();
        for model_path in model_file_paths{
            let in_bytes = fs::read(&model_path.as_ref().unwrap().path()).unwrap();
            let model = BKModel::try_from_be_bytes(&in_bytes).unwrap();
            let out_bytes = model.to_be_bytes();

            // if let Some(vtx) = model.vertices{
            //     if let Some(correct_global_norm) = vtx.preserved_global_norm {
            //     nonmatches.push(format!("{:20?}: Nonmatching Vtx global norm 0x{:04X?} != 0x{:04X?}", model_path.as_ref().unwrap().path().display(), correct_global_norm, vtx.global_norm()));
            //    }
            // }

            //header packing order
            //texture
            //display_list
            //vtx_list,
            //unk14
            //collision,
            //unk24
            //unk20
            //unk28
            //unk18
            //unk2C
            //geo_list

            // if model.header.unk_18_offset != 0 {
            //     panic!("{:?}:{:X?}", model_path.unwrap().path().display(), model.header);
            // }
            // assert_eq!(model.header.unk_a, [0;2], "{:?}",model_path.unwrap().path().display());
            // if in_bytes.len() > 0xE6C{
            // if let Some(gfx) = model.display_list{
            assert_eq!(in_bytes, out_bytes, "Error with matching {:?}: 0x{:X?}\n{:#?}", model_path.as_ref().unwrap().path().display(), in_bytes.len(), model.create_header());
            // if let Some(vtx_list) = model.display_list{
            //     assert_eq!(model.header.unk_30 as usize, vtx_list.len());
            // }
            // nonmatches.push(format!("{:X?}", model.header));
            // }
        }
        if !nonmatches.is_empty() {
            panic!("{:#?}",nonmatches);
        }
    }
}
