use std::mem::take;
use crate::utils::string_utils::mirror;

#[derive(Debug)]
#[derive(Clone)]
pub struct TextFile {
    pub name:String,
    pub format:String
}

impl TextFile {

    pub fn new(file_name:&String) -> Self {
        let mut name = String::from("");
        let mut format = String::from(""); 

        let fname_chars:Vec<char> = file_name.chars().collect();
        let mut i:usize = fname_chars.len();
        let mut dotted = false;
        
        while i>0 {
            let c:char = fname_chars[i-1];

            if dotted {
                name.push(c);
            }else if c=='.' {
                dotted = true;
            }else {
                format.push(c);
            }

            i-=1;
        }

        if name==String::from("") {
            name = std::mem::take(&mut format);
        }

        return Self {
            name: mirror(&name), 
            format: mirror(&format)
        }
    }

    pub fn set_format(self:&mut Self, new_format:&String) -> &mut Self {
        self.format = new_format.to_string();
        return self;
    }

}
