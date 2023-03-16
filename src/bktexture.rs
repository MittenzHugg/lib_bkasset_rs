use super::pixels::*;

/* BKTexture Trait can 
    - be converted between texture types 
    - from_be_bytes
    - to_be_bytes
*/

#[derive(Debug, Clone)]
pub struct BKTexture<T>
{
    pub tmem: Vec<Vec<T>>
}

impl BKTexture<IA16> {
    pub fn from_be_bytes(width: usize, height: usize, bytes: &[u8])->Self{
        let tmem = bytes.iter().cloned()
                .ia16_iter()
                .take(width*height)
                .collect::<Vec<_>>()
                .chunks_exact(width)
                .map(|row| row.to_vec())
                .collect();
        BKTexture::<IA16>{tmem}
    }

    pub fn to_be_bytes(&self)->Vec<u8>{
        self.tmem.iter().flatten().flat_map(IA16::to_be_bytes).collect()
    }
}

impl BKTexture<RGBA16> {
    pub fn from_be_bytes(width: usize, height: usize, bytes: &[u8])->Self{
        let tmem = bytes.iter().cloned()
                .rgba16_iter()
                .take(width*height)
                .collect::<Vec<_>>()
                .chunks_exact(width)
                .map(|row| row.to_vec())
                .collect();
        BKTexture::<RGBA16>{tmem}
    }

    pub fn to_be_bytes(&self)->Vec<u8>{
        self.tmem.iter().flatten().flat_map(RGBA16::to_be_bytes).collect()
    }
}

impl BKTexture<RGBA32> {
    pub fn from_be_bytes(width: usize, height: usize, bytes: &[u8])->Self{
        let tmem = bytes.iter().cloned()
                .rgba32_iter()
                .take(width*height)
                .collect::<Vec<_>>()
                .chunks_exact(width)
                .map(|row| row.to_vec())
                .collect();
        BKTexture::<RGBA32>{tmem}
    }

    pub fn to_be_bytes(&self)->Vec<u8>{
        self.tmem.iter().flatten().flat_map(RGBA32::to_be_bytes).collect()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BKTextureFormat{
    CI4,
    CI8,
    I4,
    I8,
    IA4,
    IA8,
    IA16,
    RGBA16,
    RGBA32,
    Unknown(u16),
}

#[derive(Debug)]
pub struct BKTextureHeader {
    pub offset: usize,
    pub format: BKTextureFormat,
    pub unk_6: [u8;2],
    pub width: usize,
    pub height: usize,
    pub unk_a: [u8;6],
}

impl BKTextureHeader{
    pub fn from_be_bytes(in_bytes: &[u8])->BKTextureHeader{
        let offset = u32::from_be_bytes(in_bytes[..4].try_into().unwrap()) as usize;
        let fmt_u16 = u16::from_be_bytes([in_bytes[4], in_bytes[5]]);
        let unk_6 = [in_bytes[6], in_bytes[7]];
        let format = match fmt_u16{
            0x001 => BKTextureFormat::CI4,
            0x004 => BKTextureFormat::CI8,
            0x020 => BKTextureFormat::I4,
            0x040 => BKTextureFormat::I8,
            0x080 => BKTextureFormat::IA4,
            0x100 => BKTextureFormat::IA8,
            0x400 => BKTextureFormat::RGBA16,
            0x800 => BKTextureFormat::RGBA32,
            _=> BKTextureFormat::Unknown(fmt_u16),
        };
        let width = in_bytes[8] as usize;
        let height = in_bytes[9] as usize;
        let unk_a = in_bytes[0xA.. 0x10].try_into().unwrap();
        BKTextureHeader{
            offset,
            format,
            unk_6,
            width,
            height,
            unk_a,
        }
    }

    pub fn to_be_bytes(&self)->Vec<u8>{
        let fmt : u16 = match self.format {
            BKTextureFormat::CI4 => 0x001 ,
            BKTextureFormat::CI8 => 0x004 ,
            BKTextureFormat::I4 => 0x020 ,
            BKTextureFormat::I8 => 0x040 ,
            BKTextureFormat::IA4 => 0x080 ,
            BKTextureFormat::IA8 => 0x100 ,
            BKTextureFormat::IA16 => 0x200 ,
            BKTextureFormat::RGBA16 => 0x400 ,
            BKTextureFormat::RGBA32 => 0x800 ,
            BKTextureFormat::Unknown(x)=> x,
        };
        
        vec![
            (self.offset as u32).to_be_bytes().to_vec(),
            fmt.to_be_bytes().to_vec(),
            self.unk_6.to_vec(),
            vec![self.width as u8, self.height as u8],
            self.unk_a.to_vec(),
        ].concat()
    }
}

pub trait Texture {
    //convertable to other texture types
}
// struct BKTextureCI4{
//     CI4{palette: Box<[RGBA16; 0x10]>, tmem: Vec<Vec<usize>>},
//     CI8{palette: Box<[RGBA16; 0x100]>, tmem: Vec<Vec<usize>>},
// }

// pub struct BKTexture {
//     pub format : BKTextureFormat,
//     pub palette: Option<Box<[Pixel]>>,
//     pub tmem: Vec<Vec<Box<dyn Pxl>>>
// } 

// use super::pixels;
// impl BKTexture {
//     pub fn from_be_bytes(format: &BKTextureFormat, width: usize, height: usize, bytes: &[u8])->Self{
//         let palette : Option<Box<[Pixel]>> = match format {
//             BKTextureFormat::CI4 => Some(bytes[0..0x20].iter().cloned().rgba16_iter().collect()),
//             BKTextureFormat::CI8 => Some(bytes[0..0x200].iter().cloned().rgba16_iter().collect()),
//             _ => None,
//         };

//         let tmem : Vec<Pixel> = match format {
//             BKTextureFormat::CI4 => bytes[0x20..].iter().cloned()
//                 .ci4_iter()
//                 .take(width*height).collect(),

//             BKTextureFormat::CI8 => bytes[0x20..].iter().cloned()
//                 .ci8_iter()
//                 .take(width*height).collect(),

//             BKTextureFormat::RGBA32 => bytes.iter().cloned()
//                 .rgba32_iter()
//                 .take(width*height).collect(),
//             _ => Vec::new(),
//         };

//         let tmem : Vec<Vec<Pixel>> = tmem.chunks_exact(width).map(|row| row.to_vec()).collect();

//         BKTexture{
//             format: *format,
//             palette,
//             tmem,
//         }
//     }

// }


