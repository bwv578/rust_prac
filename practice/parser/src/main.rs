mod utils;
mod core;
mod models;

use std::env;
use std::collections::HashMap;

use crate::models::text_file::TextFile;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut options:HashMap<String, String> = HashMap::new();
    let mut k:String = String::from("");

    for arg in args.iter().skip(0) {
        match arg.chars().take(2).collect::<String>().as_str() {
            "--" => {
                k = arg.chars().skip(2).collect();
            }
            _ => {
                options.insert(k.clone(), arg.clone());
            }
        }
    }

    execute(options);
}


fn execute(options:HashMap<String, String>) {

    let mut source:TextFile = match options.get(&String::from("from")) {
        Some(file_path) => TextFile::new(file_path),
        _ => {panic!("Source file is required.\nex) --from <path>");}
    };
    source.impl_as_reader();

    let mut target:TextFile = match options.get(&String::from("out")) {
        Some(file_path) => TextFile::new(file_path),
        _ => source.shell_copy()
    };

    match options.get(&String::from("as")) {
        Some(format) => {source.set_format(format);}
        _ => {}
    }

    match options.get(&String::from("to")) {
        Some(format) => {
            target
                .append_name(&String::from("."))
                .append_name(format)
                .set_format(format);
        }
        _ => {}
    }

    println!("exec source : {:#?}", source);
    println!("exec target : {:#?}", target);

    //test
    loop{
        match &mut source.iter {
            Some(iter) => {
                match &mut iter.next() {
                    Some(word) => {
                        println!("iter - next word is {}", word);
                    }
                    _ => {break;}
                }
            }
            None => {break;}
        }
    }
}
