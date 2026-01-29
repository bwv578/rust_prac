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

        while let Some(next) = self.iter.next() {
            match next.1 {

                // todo Some('\'') 과 Some('"') 각각 케이스 나눠야됨
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
                            arr.push(self.parse(StructuredData::Object(HashMap::new())));
                        },
                        StructuredData::String(str) => {
                            buf.push_str(str);
                        },
                        StructuredData::Number(num) => {
                            panic!("Invalid Format");
                        }
                    }
                },

                Some('}') => {
                    match &mut scope {
                        StructuredData::Unknown => {todo!("panic")},
                        StructuredData::Object(obj) => {},
                        StructuredData::Array(arr) => {},
                        StructuredData::String(str) => {},
                        StructuredData::Number(num) => {}
                    }
                },

                Some('[') => {
                    match &mut scope {
                        StructuredData::Unknown => {
                            scope = StructuredData::Array(Vec::new());
                        },
                        StructuredData::Object(obj) => {},
                        StructuredData::Array(arr) => {},
                        StructuredData::String(str) => {},
                        StructuredData::Number(num) => {}
                    }
                },

                Some(']') => {
                    match &mut scope {
                        StructuredData::Unknown => {todo!("panic")},
                        StructuredData::Object(obj) => {},
                        StructuredData::Array(_arr) => {
                            return scope;
                        },
                        StructuredData::String(str) => {},
                        StructuredData::Number(num) => {}
                    }
                },

                Some('\"') | Some('\'') => {
                    match &mut scope {
                        StructuredData::Unknown => {
                            scope = StructuredData::String(String::from("\""));
                        },
                        StructuredData::Object(obj) => {},
                        StructuredData::Array(arr) => {
                            arr.push(
                                self.parse(StructuredData::String(String::from("\""))))
                            ;
                        },
                        StructuredData::String(str) => {
                            str.push_str(&next.0);
                            str.push('"');
                            return scope;
                        },
                        StructuredData::Number(num) => {}
                    }
                },

                Some(':') => {
                    match &mut scope {
                        StructuredData::Unknown => {todo!("panic")},
                        StructuredData::Object(obj) => {},
                        StructuredData::Array(arr) => {},
                        StructuredData::String(str) => {
                            str.push_str(&next.0);
                            str.push(':');
                        },
                        StructuredData::Number(num) => {}
                    }
                },

                Some(',') => {
                    match &mut scope {
                        StructuredData::Unknown => {},
                        StructuredData::Object(obj) => {},
                        StructuredData::Array(arr) => {},
                        StructuredData::String(str) => {
                            str.push_str(&next.0);
                            str.push(',');
                        },
                        StructuredData::Number(num) => {}
                    }
                },

                _ => {
                    match &mut scope {
                        StructuredData::Unknown => {},
                        StructuredData::Object(obj) => {},
                        StructuredData::Array(arr) => {},
                        StructuredData::String(str) => {},
                        StructuredData::Number(num) => {}
                    }
                }

            }
        }

        scope
    }

    fn export(&mut self, structure: StructuredData) -> Result<String, Error> {
        todo!()
    }

}