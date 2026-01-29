use std::collections::{HashMap, HashSet};
use std::io::Error;
use crate::formats::format::Format;
use crate::models::intermediate_structure::StructuredData;
use crate::models::text_file::TextFile;
use crate::utils::file_utils::DelimitedIter;

static JSON_DELIMITERS: [char; 8] = ['{', '}', '[', ']', '"', '\'', ':', ','];
static JSON_CHARS_TO_IGNORE: [char; 3] = ['\n', '\r', '\t'];

pub struct JsonFormat{
    iter: DelimitedIter,
}

impl JsonFormat {
    pub fn new(file:TextFile) -> JsonFormat {
        JsonFormat{
            iter: match file.iter {
                Some(mut iterator) => {
                    iterator.set_delimiters(&JSON_DELIMITERS);
                    iterator.set_chars_to_ignore(&JSON_CHARS_TO_IGNORE);
                    iterator
                },
                _ => panic!("No iterator for the file {}", file.name)
            }
        }
    }

}

impl Format for JsonFormat {

    fn parse(&mut self, mut scope:StructuredData) -> StructuredData {
        let mut next:(String, Option<char>);
        let mut buf:String = String::new();

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
                        StructuredData::Unknown => {},
                        StructuredData::Object(obj) => {},
                        StructuredData::Array(arr) => {},
                        StructuredData::String(str) => {},
                        StructuredData::Number(num) => {}
                    }
                },

                Some('[') => {
                    match &mut scope {
                        StructuredData::Unknown => {},
                        StructuredData::Object(obj) => {},
                        StructuredData::Array(arr) => {},
                        StructuredData::String(str) => {},
                        StructuredData::Number(num) => {}
                    }
                },

                Some(']') => {
                    match &mut scope {
                        StructuredData::Unknown => {},
                        StructuredData::Object(obj) => {},
                        StructuredData::Array(arr) => {},
                        StructuredData::String(str) => {},
                        StructuredData::Number(num) => {}
                    }
                },

                Some('\"') | Some('\'') => {
                    match &mut scope {
                        StructuredData::Unknown => {},
                        StructuredData::Object(obj) => {},
                        StructuredData::Array(arr) => {},
                        StructuredData::String(str) => {},
                        StructuredData::Number(num) => {}
                    }
                },

                Some(':') => {
                    match &mut scope {
                        StructuredData::Unknown => {},
                        StructuredData::Object(obj) => {},
                        StructuredData::Array(arr) => {},
                        StructuredData::String(str) => {},
                        StructuredData::Number(num) => {}
                    }
                },

                Some(',') => {
                    match &mut scope {
                        StructuredData::Unknown => {},
                        StructuredData::Object(obj) => {},
                        StructuredData::Array(arr) => {},
                        StructuredData::String(str) => {},
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