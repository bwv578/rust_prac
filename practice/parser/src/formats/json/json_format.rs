use std::io::Error;
use crate::formats::format::Format;
use crate::models::intermediate_structure::IntermediateStructure;
use crate::models::text_file::TextFile;

pub struct JsonFormat{
    //file_to_read:Option<Box<TextFile>>,
    //struct_to_export: Option<Box<IntermediateStructure>>
}

impl Format for JsonFormat {
    fn construct(&mut self, file: TextFile) -> IntermediateStructure {
        todo!()
    }

    fn export(&mut self, structure: IntermediateStructure) -> Result<String, Error> {
        todo!()
    }
}