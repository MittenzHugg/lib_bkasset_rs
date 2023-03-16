use std::error::Error;

#[derive(Debug)]
pub struct BKMesh{
    pub uid: u16,
    pub vtx_indices: Vec<usize>
}

impl BKMesh{
    pub fn try_from_be_bytes(bytes: &[u8])->Result<Self, Box<dyn Error>>{
        let vtx_count = u16::from_be_bytes(bytes[2..4].try_into()?) as usize;
        let vtx_indices = bytes[4..].chunks_exact(2)
            .map(|b| b.try_into().map(u16::from_be_bytes).map(usize::from))
            .take(vtx_count)
            .collect::<Result<Vec<usize>,_>>()?;
        let this = Self{
            uid : u16::from_be_bytes(bytes[0..2].try_into()?),
            vtx_indices
        };
        #[cfg(feature = "test_byte_matching")]assert_eq!(bytes[..this.size()], this.to_be_bytes(), "\n{:#?}", this);
        return Ok(this)
    }

    pub fn to_be_bytes(&self)->Vec<u8>{
        [
            self.uid.to_be_bytes().as_slice(),
            (self.vtx_indices.len() as u16).to_be_bytes().as_slice(),
            self.vtx_indices.iter()
                .map(|&val| val as u16)
                .flat_map(u16::to_be_bytes)
                .collect::<Vec<_>>().as_slice()
        ].concat().try_into().unwrap()
    }

    pub fn size(&self)->usize{
        4 + 2*self.vtx_indices.len()
    }
}

#[derive(Debug)]
pub struct BKMeshList{
    pub meshes:Vec<BKMesh>
}

impl BKMeshList{
    pub fn try_from_be_bytes(bytes: &[u8])->Result<Self, Box<dyn Error>>{
        let mesh_count = u16::from_be_bytes(bytes[0..2].try_into()?);
        let mut this = Self{
            meshes: Vec::new()
        };

        let mut offset = 2;
        for _ in 0 .. mesh_count{
            let i_mesh = BKMesh::try_from_be_bytes(&bytes[offset..])?;
            offset += i_mesh.size();
            this.meshes.push(i_mesh);
        }
        #[cfg(feature = "test_byte_matching")]assert_eq!(bytes[..this.size()], this.to_be_bytes(), "\n{:#?}", this);

        return Ok(this)
    }

    pub fn to_be_bytes(&self)->Vec<u8>{
        let mut bytes = Vec::with_capacity(self.size());
        bytes = [
            (self.meshes.len() as u16).to_be_bytes().as_slice(),
            self.meshes.iter()
                .flat_map(BKMesh::to_be_bytes)
                .collect::<Vec<u8>>().as_slice()
        ].concat();
        bytes.resize(self.size(), 0);
        return bytes
    }

    pub fn size(&self)->usize{
        let size = 2 + self.meshes.iter()
            .map(BKMesh::size)
            .sum::<usize>();
        (size + 7) & !7 //align
    }
}