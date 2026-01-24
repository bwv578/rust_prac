use std::env;

mod utils;
mod models {
    pub mod middle_format;
    pub mod text_file;
}

use crate::models::text_file::TextFile;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let mut source:&String;
    let mut target:&String;

    match args.get(1) {
        Some(option) => {
            handle_option(option);
        }
        _ => {
            println!("else.");
        }
    }
}

fn handle_option(option:&String){
    println!("handle option parameter : {}", option);
    let new_file:TextFile = TextFile::new(option);
    println!("new file - {:#?}", new_file);
}
