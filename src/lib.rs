pub mod bkdialog;
pub mod bkstring;
pub mod bkmodel;
pub mod bktexture;
pub mod pixels;
pub mod error;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ImgFmt{
    CI4,
    CI8,
    I4,
    I8,
    RGBA16,
    RGBA32,
    IA4,
    IA8,
    Unknown(u16),
}

pub enum AssetType{
    Animation,
    Binary,
    DemoInput,
    Dialog,
    GruntyQuestion,
    LevelSetup,
    Midi,
    Model,
    QuizQuestion,
    Sprite(ImgFmt),
}

pub struct AssetFolder{

}