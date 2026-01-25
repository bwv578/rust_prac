use std::mem::take;
use crate::utils::string_utils::mirror;

#[derive(Debug)]
#[derive(Clone)]
pub struct TextFile {
    pub name:String,
    pub format:String
}

impl TextFile {

    /*pub fn new(file_name:&String) -> Self {
        let mut name = String::from("");
        let mut format = String::from(""); 

        let fname_chars:Vec<char> = file_name.chars().collect();
        let mut i:usize = fname_chars.len();
        let mut dotted:bool = false;
        
        while i>0 {
            let c:char = fname_chars[i-1];

            if dotted {
                name.push(c);
            }else if c=='.' {
                if i==1 {
                    name = std::mem::take(&mut format);
                    name.push(c);
                }else {
                    match fname_chars.get(i-2) {
                        Some('/')|Some('.') => {
                            name = std::mem::take(&mut format);
                            name.push(c);
                        }
                        _ => {}
                    }
                }
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
    }*/

    pub fn new(file_name:&String) -> Self {
        let mut inferred_format:String = String::from("");

        let vectorized:Vec<char> = file_name.chars().collect();
        let mut i:usize = vectorized.len()-1;

        while i>=0 {
            let c:char = vectorized[i];
            if c == '.' {break;}

            inferred_format.push(c);
            i-=1;
        }

        return Self {
            name: file_name.clone(),
            format: mirror(&inferred_format)
        }
    }

    pub fn set_name(self:&mut Self, new_name:&String) -> &mut Self {
        self.name = new_name.to_string();
        return self;
    }

    pub fn set_format(self:&mut Self, new_format:&String) -> &mut Self {
        self.format = new_format.to_string();
        return self;
    }

    pub fn append_name(self:&mut Self, suffix:&String) -> &mut Self {
        self.name.push_str(suffix);
        return self;
    }
}
