use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
pub struct IntermediateStructure {
    pub name: String,
    pub data: HashMap<String, IntermediateStructure>
}