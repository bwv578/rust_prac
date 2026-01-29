use std::collections::HashMap;

/*#[derive(Debug)]
#[derive(Clone)]
pub struct IntermediateStructure {
    pub name: String,
    pub data: Box<DataType>
}

impl IntermediateStructure {

    pub fn new() -> Self {
        return Self {
            name: String::new(),
            data: Box::new(DataType::Unknown)
        };
    }

    pub fn set_data(self:&mut Self, data:DataType) -> &mut Self {
        self.data = Box::new(data);
        return self;
    }

}*/

#[derive(Debug)]
#[derive(Clone)]
pub enum StructuredData {
    Object(HashMap<String, StructuredData>),
    Array(Vec<StructuredData>),
    String(String),
    Number(f64),
    Unknown
}