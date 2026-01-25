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
                                    return Some(self.word.clone());
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