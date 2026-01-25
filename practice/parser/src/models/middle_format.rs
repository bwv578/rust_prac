use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
pub struct MiddleFormat {
    pub name: String,
    pub data: HashMap<String, String>
}
