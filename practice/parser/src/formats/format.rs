use std::io::Error;

use crate::models::text_file::TextFile;
use crate::models::intermediate_structure::StructuredData;
use crate::utils::file_utils::DelimitedIter;

pub trait Format{
    fn parse (&mut self, scope:StructuredData) -> StructuredData;
    fn export (&mut self, structure:StructuredData) -> Result<String, Error>;
}