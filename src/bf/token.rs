use super::common::Instruction::*;

use std::io::Read;

pub struct Tokenizer<T: Read> {
    source: T,
    eof: bool
}

impl<T: Read> Tokenizer<T> {
    pub fn new(source: T) -> Self {
        Self { source, eof: false }
    }
}

impl<T: Read> std::iter::Iterator for Tokenizer<T> {
    type Item = super::common::Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eof {
            return None
        }
        let mut b = [0u8; 1];
        loop {
            match self.source.read(&mut b) {
                Ok(0) => {},
                Err(_) => {},
                Ok(_) => {
                    return Some(match b[0] {
                        b'>' => MoveRight,
                        b'<' => MoveLeft,
                        b'+' => Increment,
                        b'-' => Decrement,
                        b'.' => Output,
                        b',' => Input,
                        b'[' => LeftLoop,
                        b']' => RightLoop,
                        _ => continue
                    })
                }
            }
            self.eof = true;
            return Some(EOF);
        }
    }
}