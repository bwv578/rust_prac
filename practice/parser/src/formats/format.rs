use std::io::Error;

use crate::models::text_file::TextFile;
use crate::models::intermediate_structure::IntermediateStructure;
use crate::utils::file_utils::DelimitedIter;

pub trait Format{
    fn construct<T> (&mut self, file:TextFile) -> IntermediateStructure<T>;
    fn parse<T> (&mut self, iter:&mut DelimitedIter) -> IntermediateStructure<T>;
    fn export<T> (&mut self, structure:IntermediateStructure<T>) -> Result<String, Error>;
}