use std::collections::HashMap;
use crate::models::text_file::TextFile;


pub fn parse_options(args:&Vec<String>) -> HashMap<String, String>{
    let mut options:HashMap<String, String> = HashMap::new();
    let mut k:String = String::new();

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

    return options;
}

pub fn execute(options:HashMap<String, String>) {

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
    let mut iter = source.iter.as_mut().expect("no source reader.");

    loop{
        iter.set_delimiters(&['{', '}', '[', ']', ':', ',', '"']); // json 구분자 테스트
        iter.set_chars_to_ignore(&['\n', '\t', '\r', '\x0B', '\x0C', '\x0A', '\x0A']);
        match &mut iter.next() {
            Some(r) => {
                let delimiter:char = r.1.unwrap_or_else(|| ' ');
                println!("String:{} | delimiter:{}", r.0, delimiter);
            }
            _ => {break;}
        }
    }
}