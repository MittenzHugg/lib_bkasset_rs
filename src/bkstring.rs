use yaml_rust::{Yaml};


pub fn string_to_vecu8(string: &str) -> Vec<u8>{
    let mut string = string.as_bytes().to_vec();
    let mut squig_indx : Vec<usize> = string.windows(2)
        .enumerate()
        .filter(|(_, win)|{match win {[0xC3, 0xBD]=> true, _=>false,} })
        .map(|(i, _)|{i})
        .collect();
    squig_indx.reverse();
    for i in squig_indx{
        string[i] = 0xFD;
        string.remove(i+1);
    }
    string.push(0);
    return string
}

pub fn vecu8_to_string(bytes: &Vec<u8>) -> String{
    let mut out : String = String::new();
    for b in &bytes[..bytes.len() - 1]{
        let ch = *b as char;
        if !ch.is_ascii() || *b < 0x20 {
            out += format!("\\x{:02X}", ch as u8).as_str();
        }
        else{
            out.push(ch);
        }
    }
    return out
}

#[derive(Clone)]
pub struct BKString{
    pub cmd: u8,
    pub string: Vec<u8>,
}

impl BKString{
    pub fn new() -> BKString{
        BKString{cmd: 4, string: vec![0]}
    }

    pub fn from_yaml(yaml: &Yaml) -> BKString{
        let cmd = yaml["cmd"].as_i64().unwrap() as u8;
        let string = string_to_vecu8(&yaml["string"].as_str().unwrap());            
        
        BKString{cmd : cmd, string: string}
    }
}