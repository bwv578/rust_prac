use std::collections::{HashMap, HashSet};
use std::io::Error;
use crate::formats::format::Format;
use crate::models::structured_data::StructuredData;
use crate::models::text_file::TextFile;
use crate::utils::file_utils::DelimitedIter;

static JSON_DELIMITERS: [char; 8] = ['{', '}', '[', ']', '"', '\'', ':', ','];
static JSON_CHARS_TO_IGNORE: [char; 3] = ['\n', '\r', '\t'];

pub struct JsonFormat{
    iter: DelimitedIter,
}

enum Target{
    Key,
    Value
}

impl JsonFormat {
    pub fn new(file:TextFile) -> Self {
        let mut json_iter = file.iter.expect("No iterator for the file.");
        json_iter.set_delimiters(&JSON_DELIMITERS);
        json_iter.set_chars_to_ignore(&JSON_CHARS_TO_IGNORE);
        JsonFormat{iter: json_iter,}
    }
}

impl Format for JsonFormat {

    fn parse(&mut self, mut scope:StructuredData) -> StructuredData {
        let mut buf:String = String::new();
        let mut target:Target = Target::Key;

        while let Some(next) = self.iter.next() {
            match next.1 {

                Some('{') => {
                    match &mut scope {
                        StructuredData::Unknown => {
                            scope = StructuredData::Object(HashMap::new());
                        },
                        StructuredData::Object(obj) => {
                            obj.insert(
                                std::mem::take(&mut buf),
                                self.parse(StructuredData::Object(HashMap::new()))
                            );
                        },
                        StructuredData::Array(arr) => {
                            arr.push(
                                self.parse(
                                    StructuredData::Object(HashMap::new())
                                )
                            );
                        },
                        StructuredData::String(str) => {
                            str.push_str(&next.0);
                            str.push('{');
                        },
                        StructuredData::Number(_num) => { panic!("Invalid Format"); }
                    }
                },

                Some('}') => {
                    match &mut scope {
                        StructuredData::Unknown => { panic!("Invalid Format"); },
                        StructuredData::Object(_obj) => {
                            return scope;
                        },
                        StructuredData::Array(_arr) => { panic!("Invalid Format"); },
                        StructuredData::String(str) => {
                            str.push_str(&next.0);
                            str.push('}');
                        },
                        StructuredData::Number(num) => { panic!("Invalid Format"); },
                    }
                },

                Some('[') => {
                    match &mut scope {
                        StructuredData::Unknown => {
                            scope = StructuredData::Array(Vec::new());
                        },
                        StructuredData::Object(obj) => {
                            obj.insert(
                                std::mem::take(&mut buf),
                                self.parse(StructuredData::Array(Vec::new()))
                            );
                        },
                        StructuredData::Array(arr) => {
                            arr.push(
                                self.parse(StructuredData::Array(Vec::new()))
                            );
                        },
                        StructuredData::String(str) => {
                            str.push_str(&next.0);
                            str.push(']');
                        },
                        StructuredData::Number(num) => { panic!("Invalid Format"); }
                    }
                },

                Some(']') => {
                    match &mut scope {
                        StructuredData::Unknown => { panic!("Invalid Format"); },
                        StructuredData::Object(_obj) => { panic!("Invalid Format"); },
                        StructuredData::Array(_arr) => { return scope; },
                        StructuredData::String(str) => {
                            str.push_str(&next.0);
                            str.push('}');
                        },
                        StructuredData::Number(num) => { panic!("Invalid Format"); }
                    }
                },

                Some('\"') | Some('\'') => {
                    match &mut scope {
                        StructuredData::Unknown => { panic!("Invalid Format"); },
                        StructuredData::Object(obj) => {
                            match target {
                                Target::Key => {
                                    buf = self
                                        .parse(StructuredData::String(String::from(next.1.unwrap())))
                                        .take_string()
                                        .expect("Invalid format.");
                                },
                                Target::Value => {
                                    obj.insert(
                                        std::mem::take(&mut buf),
                                        self.parse(
                                            StructuredData::String(String::from(next.1.unwrap()))
                                        )
                                    );
                                }
                            }
                        },
                        StructuredData::Array(arr) => {
                            arr.push(
                                self.parse(StructuredData::String(String::from(next.1.unwrap())))
                            );
                        },
                        StructuredData::String(str) => {
                            str.push_str(&next.0);
                            match str.chars().next() {
                                Some(opener) => { str.push(opener); }
                                _ => { panic!("어???"); }
                            }
                            return scope;
                        },
                        StructuredData::Number(_num) => { panic!("Invalid Format"); },
                    }
                },

                Some(':') => {
                    match &mut scope {
                        StructuredData::Unknown => { panic!("Invalid Format"); },
                        StructuredData::Object(_obj) => {
                            buf.push_str(&next.0.trim());
                            if buf.is_empty() {
                                panic!("Invalid Format : Key is empty");
                            }
                        },
                        StructuredData::Array(_arr) => { panic!("Invalid Format"); },
                        StructuredData::String(str) => {
                            str.push_str(&next.0);
                            str.push(':');
                        },
                        StructuredData::Number(num) => { panic!("Invalid Format"); }
                    }
                },

                Some(',') => {
                    match &mut scope {
                        StructuredData::Unknown => { panic!("Invalid Format"); },
                        StructuredData::Object(obj) => {
                            if !next.0.trim().is_empty() {
                                
                                if next.0.trim() == "true" || next.0.trim() == "false" {
                                    obj.insert(
                                        std::mem::take(&mut buf),
                                        StructuredData::String( String::from(next.0.trim()) )
                                    );
                                }else {
                                    obj.insert(
                                        std::mem::take(&mut buf),
                                        StructuredData::Number(next.0.trim().parse::<f64>().unwrap())
                                    );
                                }

                            }else {
                                buf.clear();
                            }
                        },
                        StructuredData::Array(arr) => {
                            if !next.0.trim().is_empty() { // boolean 또는 숫자
                                if next.0.trim() == "true" || next.0.trim() == "false" {
                                    arr.push(
                                        StructuredData::String( String::from(next.0.trim()) )
                                    );
                                }else{
                                    arr.push(
                                        StructuredData::Number(next.0.trim().parse::<f64>().unwrap())
                                    )
                                }
                            }
                        },
                        StructuredData::String(str) => {
                            str.push_str(&next.0);
                            str.push(',');
                        },
                        StructuredData::Number(num) => {
                            // todo 이거 필요 없는거같은데
                            *num = next.0.trim().parse().unwrap();
                            return scope;
                        }
                    }
                },

                None => { // 마지막 값
                    /*match &mut scope {
                        StructuredData::Unknown => { panic!("Invalid Format"); },
                        StructuredData::Object(obj) => {
                        },
                        StructuredData::Array(arr) => {},
                        StructuredData::String(str) => {},
                        StructuredData::Number(num) => {}
                    }*/
                    // 항상 ] 또는 }로 끝나야 하므로 포멧 오류 ?
                    panic!("Invalid Format");
                }
                _ => {panic!("세상에 이런일이");}
            }
        }

        return scope
    }

    fn export(&mut self, structure: StructuredData) -> Result<String, Error> {
        todo!()
    }

}

impl StructuredData {

    fn take_string(&mut self) -> Option<String> {
        match self {
            StructuredData::String(str) => {
                return Some(std::mem::take(str));
            }
            _ => None
        }
    }

}