use std::io::Error;

use crate::models::text_file::TextFile;
use crate::models::intermediate_structure::IntermediateStructure;
use crate::utils::file_utils::DelimitedIter;

pub trait Format{
    fn construct (&mut self, file:TextFile) -> IntermediateStructure;
    fn parse (&mut self, iter:&mut DelimitedIter) -> IntermediateStructure;
    fn export (&mut self, structure:IntermediateStructure) -> Result<String, Error>;
}