use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
pub struct IntermediateStructure<T> {
    pub name: String,
    pub data: Option<Box<T>>
}

impl<T> IntermediateStructure<T> {

    pub fn new() -> Self {
        return Self {
            name: String::new(),
            data: None
        };
    }

    pub fn set_data(self:&mut Self, data:T) -> &mut Self {
        self.data = Some(Box::new(data));
        return self;
    }

}