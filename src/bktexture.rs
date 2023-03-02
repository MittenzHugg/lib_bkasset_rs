


#[derive(Debug)]
pub enum BKTextureType{
    CI4,
    CI8,
    I4,
    I8,
    IA4,
    IA8,
    RGBA16,
    RGBA32,
    Unknown(u16),
}

#[derive(Debug)]
pub struct BKTextureHeader {
    pub offset: usize,
    pub format: BKTextureType,
    // pub unk6: [u8;2],
    pub width: usize,
    pub height: usize,
    // pub unkA: [u8;6],
}

impl BKTextureHeader{
    pub fn from_be_bytes(in_bytes: &[u8])->BKTextureHeader{
        let offset = u32::from_be_bytes(in_bytes[..4].try_into().unwrap()) as usize;
        let fmt_u16 = u16::from_be_bytes([in_bytes[4], in_bytes[5]]);
        let format = match fmt_u16{
            0x001 => BKTextureType::CI4,
            0x004 => BKTextureType::CI8,
            0x020 => BKTextureType::I4,
            0x040 => BKTextureType::I8,
            0x080 => BKTextureType::IA4,
            0x100 => BKTextureType::IA8,
            0x400 => BKTextureType::RGBA16,
            0x800 => BKTextureType::RGBA32,
            _=> BKTextureType::Unknown(fmt_u16),
        };
        let width = in_bytes[8] as usize;
        let height = in_bytes[9] as usize;
        BKTextureHeader{
            offset,
            format,
            width,
            height,
        }
    }
}

