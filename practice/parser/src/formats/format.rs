use std::io::Error;

use crate::models::text_file::TextFile;
use crate::models::intermediate_structure::IntermediateStructure;


pub trait Format{
    fn construct (&mut self, file:TextFile) -> IntermediateStructure;
    fn export (&mut self, structure:IntermediateStructure) -> Result<String, Error>;
}