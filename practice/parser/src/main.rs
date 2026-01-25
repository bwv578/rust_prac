mod utils;
mod models {
    pub mod middle_format;
    pub mod text_file;
}

use std::env;
use std::collections::HashMap;

use crate::models::text_file::TextFile;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    
    let mut options:HashMap<String, String> = HashMap::new();
    let mut k:String = String::from("");

    for arg in args.iter().skip(0) {
        match arg.chars().next() {
            Some('-') => {
                k = arg.chars().skip(1).collect();
            }
            Some(_c) => {
                options.insert(k.clone(), arg.clone());
            }
            _ => {}
        }
    }

    println!("check map : {:?}", options);
    execute(options);
}


fn execute(options:HashMap<String, String>) {

    //let source = TextFile::new(&options.get(&String::from("from")));
    let source:TextFile = match options.get(&String::from("from")) {
        Some(v) => TextFile::new(v),
        _ => {panic!("Invalid option value.");}
    };

    let target:TextFile = match options.get(&String::from("to")) {
        Some(v) => TextFile::new(v),
        _ => source.clone()
    };

    //let target = TextFile::new(&options.get(&String::from("target")));
    println!("exec source : {:#?}", source);
    println!("exec target : {:#?}", target);
}
