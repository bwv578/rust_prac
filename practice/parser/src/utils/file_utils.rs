use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Error, Read, BufWriter};

#[derive(Debug)]
pub struct WordIter {
    word: String,
    reader: Option<BufReader<File>>
}

impl WordIter {
    pub fn new(file_path:&String) -> Result<WordIter, Error> {
        let file_to_read:File = File::open(file_path)?;
        return Ok(
            Self {
                word: String::from(""),
                reader: Some(BufReader::new(file_to_read))
            }
        )
    }

    pub fn next(&mut self) -> Option<String> {
        self.word.clear();
        let mut buff = [0u8; 1];

        loop{
            match &mut self.reader {
                Some(reader) => {
                    match reader.read(&mut buff) {
                        Ok(0) => {
                            if self.word.is_empty() {
                                return None;
                            } else {
                                return Some(self.word.clone());
                            }
                        },
                        Ok(_n) => {
                            match buff[0] as char {
                                ' ' | '\t' | '\n' => {
                                    if !self.word.is_empty() {
                                        return Some(self.word.clone());
                                    }
                                },
                                _ => {
                                    self.word.push(buff[0] as char);
                                }
                            }
                        }
                        Err(e) => {panic!("File Error: {}", e);}
                    }
                }
                _ => {panic!("There's no file reader for this instance.");}
            }
        }
    }
}

#[derive(Debug)]
pub struct DelimitedIter {
    out: (String, Option<char>),
    delimiters: HashSet<char>,
    reader: Option<BufReader<File>>
}

impl DelimitedIter {
    pub fn new(file_path:&String) -> Result<DelimitedIter, Error> {
        let file_to_read:File = File::open(file_path)?;
        return Ok(
            Self {
                out: (String::new(), None),
                delimiters: HashSet::new(),
                reader: Some(BufReader::new(file_to_read))
            }
        )
    }
    pub fn set_delimiters(&mut self, delimiters: &[char]) -> &mut Self {
        self.delimiters.clear();
        for delimiter in delimiters {
            self.delimiters.insert(*delimiter);
        }
        return self;
    }

    pub fn next(&mut self) -> Option<(String, Option<char>)> {
        self.out = (String::new(), None);
        let mut buff = [0u8; 1];

        loop {
            let reader = self.reader.as_mut().expect("No file reader");
            match reader.read(&mut buff) {
                Ok(0) => {
                    if self.out.0.is_empty() {
                        return None;
                    }else{
                        return Some(std::mem::take(&mut self.out));
                    }
                }
                Ok(_) => {
                    let c:char = buff[0] as char;
                    if self.delimiters.contains(&c) {
                        self.out.1 = Some(c);
                        return Some(std::mem::take(&mut self.out));
                    }else{
                        self.out.0.push(c);
                    }
                }
                Err(e) => {panic!("File Error: {}", e);}
            }
        }
    }

}