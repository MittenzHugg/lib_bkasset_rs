use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct BKAnimatedTexture{
    pub frame_size: i16,
    pub frame_count: i16,
    pub framerate_fps: f32,
}

impl Default for BKAnimatedTexture{
    fn default() -> Self {
        Self{
            frame_size : 0, 
            frame_count : 0,
            framerate_fps: 0.0,
        }
    }
}

impl BKAnimatedTexture {
    pub fn from_be_bytes(bytes: [u8; 8])->Self{
        let this = Self{
            frame_size: i16::from_be_bytes([bytes[0], bytes[1]]),
            frame_count: i16::from_be_bytes([bytes[2], bytes[3]]),
            framerate_fps: f32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
        };
        #[cfg(feature = "test_byte_matching")]assert_eq!(bytes, this.to_be_bytes(), "\n{:#?}", this);
        return this

    }

    pub fn to_be_bytes(&self)->[u8; 8]{
        [
            self.frame_size.to_be_bytes().as_slice(),
            self.frame_count.to_be_bytes().as_slice(),
            self.framerate_fps.to_be_bytes().as_slice()
        ].concat().try_into().unwrap()
    }
}

#[derive(Debug)]
pub struct BKAnimatedTextureList{
    pub values: [Option<BKAnimatedTexture>; 4],
}

impl BKAnimatedTextureList{
    pub fn from_be_bytes(bytes: [u8; 0x20])->Self{
        let this = Self{
            values: bytes[0..0x20].chunks_exact(0x8)
            .map(|bytes| BKAnimatedTexture::from_be_bytes(bytes.try_into().unwrap()))
            .map(|txtr| match txtr.frame_size { 0 => None, _ => Some(txtr)})
            .take(4).collect::<Vec<_>>().try_into().unwrap()
        };
        #[cfg(feature = "test_byte_matching")]assert_eq!(bytes, this.to_be_bytes(), "\n{:#?}", this);
        return this
    }

    pub fn to_be_bytes(&self)->[u8; 0x20]{
        self.values.iter()
            .map(|maybe_txtr| maybe_txtr.clone().unwrap_or_default())
            .flat_map(|anim_txtr|anim_txtr.to_be_bytes())
            .collect::<Vec<_>>()
            .try_into().unwrap()
    }

    pub fn size(&self)->usize{0x20}
}

impl Deref for BKAnimatedTextureList{
    type Target = [Option<BKAnimatedTexture>; 4];
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl DerefMut for BKAnimatedTextureList{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}