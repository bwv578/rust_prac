use std::io::Error;
use crate::formats::format::Format;
use crate::models::intermediate_structure::IntermediateStructure;
use crate::models::text_file::TextFile;
use crate::utils::file_utils::DelimitedIter;

pub struct JsonFormat{
    //file_to_read:Option<Box<TextFile>>,
    //intermediate_struct: Option<Box<IntermediateStructure<T>>>
}

impl Format for JsonFormat {
    fn construct<U>(&mut self, mut file: TextFile) -> IntermediateStructure<U> {
        let mut iter: DelimitedIter = match file.iter {
            Some(iterator) => iterator,
            None => {panic!("Error while constructing {}.\n (No file iterator.)", file.name)}
        };
        iter.set_delimiters(&['\n', '\r', 't']);
        return self.parse(&mut iter);
    }

    fn parse<U>(&mut self, iter: &mut DelimitedIter) -> IntermediateStructure<U> {
        let mut object:IntermediateStructure<U> = IntermediateStructure::new();

        let mut next:(String, Option<char>);

        let mut mode:Mode = Mode::Unknown;
        let mut target:Target = Target::Key;

        loop {
            next = match iter.next() {
                Some(out) => out,
                None => {break;}
            };

            match next.1 {
                Some('{') => {
                    match mode {
                        Mode::Unknown => {
                            mode = Mode::MapObject
                        },
                        _ => {}
                    }
                    match target {
                        Target::Key => {},
                        _ => {}
                    }
                },
                Some('}') => {
                    match target {
                        Target::Key => {},
                        Target::Value => {}
                    }
                    return object;
                },
                Some('[') => {},
                Some(']') => {},
                Some('\"') | Some('\'') => {},
                Some(':') => {},
                Some(',') => {},
                _ => {}
            }
        }

        return object;
    }

    fn export<U>(&mut self, structure: IntermediateStructure<U>) -> Result<String, Error> {
        todo!()
    }

}


enum Mode {
    Unknown,
    MapObject,
    ArrayObject,
    Key,
    StringValue,
    NumberValue,
}

enum Target {
    Key,
    Value
}