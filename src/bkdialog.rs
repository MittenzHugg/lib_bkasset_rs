use std::fs::{self, File};
use std::io::{self, Write};
use yaml_rust::{Yaml, YamlLoader};
use std::error::Error;

use super::bkstring::{BKString, vecu8_to_string};

pub struct BKDialog{
    pub bottom: Vec<BKString>,
    pub top: Vec<BKString>,
}

impl BKDialog{
    pub fn new()->BKDialog{
        BKDialog{bottom : vec![BKString::new()], top : vec![BKString::new()]}
    }

    //bin -> BKDialog
    pub fn from_bytes(in_bytes: &[u8])->BKDialog{
        let mut offset : usize = 3;
            
        let mut bottom = Vec::new();
        let bottom_size : u8 = in_bytes[offset];
        offset += 1;
        for _ in 0..bottom_size{
            let cmd : u8 = in_bytes[offset];
            let str_size : u8 = in_bytes[offset + 1];
            let i_string = BKString{cmd : cmd, string : in_bytes[offset + 2 .. offset + 2 + str_size as usize].to_vec()};
            bottom.push(i_string);
            offset += 2 + str_size as usize;
        }

        let mut top = Vec::new();
        let top_size : u8 = in_bytes[offset];
        offset += 1;
        for _ in 0..top_size{
            let cmd : u8 = in_bytes[offset];
            let str_size : u8 = in_bytes[offset + 1];
            let i_string = BKString{cmd : cmd, string : in_bytes[offset + 2 .. offset + 2 + str_size as usize].to_vec()};
            top.push(i_string);
            offset += 2 + str_size as usize;
        }

        return BKDialog{ bottom, top};
    }

    //yaml -> BKDialog
    pub fn read(path: &str) -> Result<BKDialog, Box<dyn Error>>{
        let doc = &YamlLoader::load_from_str(&fs::read_to_string(path).expect("could not open yaml"))?[0];
        let bottom : Vec<BKString> = match doc["bottom"].as_vec() {
            Some(bottom_obj) => bottom_obj.iter().map(BKString::from_yaml).collect(),
            None => vec![BKString::new()],
        };

        let top : Vec<BKString> = match doc["bottom"].as_vec() {
            Some(top_obj) => top_obj.iter().map(BKString::from_yaml).collect(),
            None => vec![BKString::new()],
        };
        Ok(BKDialog{bottom, top})
    }

    // BKDialog -> bin
    pub fn to_bytes(&self)->Vec<u8>{
        let mut out :Vec<u8> = vec![0x01, 0x03, 0x00];
        out.push(self.bottom.len() as u8);
        for text in self.bottom.iter(){
            out.push(text.cmd);
            out.push(text.string.len() as u8);
            out.append(&mut text.string.clone());
        }
        out.push(self.top.len() as u8);
        for text in self.top.iter(){
            out.push(text.cmd);
            out.push(text.string.len() as u8);
            out.append(&mut text.string.clone());
        }
        return out;
    }

    // BKDialog -> yaml
    pub fn write(&self, path: &str) -> Result<(), io::Error>{
        let mut bin_file = File::create(path)?;
        
        writeln!(bin_file, "bottom:")?;
        for text in self.bottom.iter(){
            writeln!(bin_file,"  - {{ cmd: 0x{:02X}, string: \"{}\"}}", text.cmd, vecu8_to_string(&text.string))?
        }
        writeln!(bin_file, "top:")?;
        for text in self.top.iter(){
            writeln!(bin_file,"  - {{ cmd: 0x{:02X}, string: \"{}\"}}", text.cmd, vecu8_to_string(&text.string))?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
