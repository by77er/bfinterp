#![allow(dead_code)]

use super::common::Token;
use Token::*;

use std::io::{Read, Write};
use std::iter::Iterator;

use std::result::Result;

pub struct BFInterpreter<'a, T: Read, U: Write, V: Iterator<Item = Token>> {
    read_stream: &'a mut T,
    write_stream: &'a mut U,
    token_source: V,
    token_buffer: Vec<Token>,
    memory: Vec<u8>,
    mem_idx: usize,
    stack: Vec<usize>,
    pc: usize,
}

impl<'a, T: Read, U: Write, V: Iterator<Item = Token>> BFInterpreter<'a, T, U, V> {
    pub fn new(src: V, input: &'a mut T, output: &'a mut U) -> Self {
        Self::with_capacity(src, input, output, 30000)
    }

    pub fn with_capacity(src: V, input: &'a mut T, output: &'a mut U, capacity: usize) -> Self {
        Self {
            read_stream: input,
            write_stream: output,
            token_source: src,
            token_buffer: Vec::new(),
            memory: vec![0; capacity],
            mem_idx: 0,
            stack: Vec::new(),
            pc: 0,
        }
    }

    #[inline]
    fn read_tokens(&mut self, n: usize) -> Result<usize, ()> {
        for _ in 0..n {
            if let Some(i) = self.token_source.next() {
                self.token_buffer.push(i)
            } else {
                return Err(());
            }
        }
        Ok(n)
    }

    fn get_token(&mut self, idx: usize) -> Result<&Token, ()> {
        if self.token_buffer.len() < (idx + 1) {
            match self.read_tokens((idx + 1) - self.token_buffer.len()) {
                Ok(_) => {}
                Err(()) => return Err(()),
            }
        }
        Ok(&self.token_buffer[idx])
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        loop {
            let ins = match self.get_token(self.pc) {
                Ok(i) => i,
                Err(_) => return Err("Failed to read Token."),
            };
            match ins {
                MoveRight => {
                    if self.mem_idx == self.memory.len() - 1 {
                        self.mem_idx = 0;
                    } else {
                        self.mem_idx += 1;
                    }
                }
                MoveLeft => {
                    if self.mem_idx == 0 {
                        self.mem_idx = self.memory.len() - 1;
                    } else {
                        self.mem_idx -= 1;
                    }
                }
                Increment => {
                    self.memory[self.mem_idx] = self.memory[self.mem_idx].wrapping_add(1);
                }
                Decrement => {
                    self.memory[self.mem_idx] = self.memory[self.mem_idx].wrapping_sub(1);
                }
                Output => {
                    match self
                        .write_stream
                        .write(&self.memory[self.mem_idx..self.mem_idx + 1])
                    {
                        Ok(_) => {}
                        Err(_) => return Err("Failed to write to output."),
                    }
                }
                Input => {
                    match self
                        .read_stream
                        .read(&mut self.memory[self.mem_idx..self.mem_idx + 1])
                    {
                        Ok(_) => {}
                        Err(_) => return Err("Failed to read from input."),
                    }
                }
                LeftLoop => {
                    if self.memory[self.mem_idx] == 0 {
                        // jump forward
                        let mut count = 1;
                        while count != 0 {
                            self.pc += 1;
                            let stru = match self.get_token(self.pc) {
                                Ok(i) => i,
                                Err(_) => return Err("Failed to read Token."),
                            };
                            match stru {
                                EOF => {
                                    return Err("Reached EOF while searching for ]. Unmatched [.")
                                }
                                LeftLoop => count += 1,
                                RightLoop => count -= 1,
                                _ => continue,
                            }
                        }
                    } else {
                        // continue and push to the stack
                        self.stack.push(self.pc);
                    }
                }
                RightLoop => {
                    // Establish that stack has an element
                    if self.stack.is_empty() {
                        return Err("Stack is unexpectedly empty. Unmatched ].");
                    }
                    if self.memory[self.mem_idx] != 0 {
                        // jump backward
                        self.pc = self.stack[self.stack.len() - 1];
                    } else {
                        // continue and pop the stack
                        self.stack.pop().unwrap();
                    }
                }
                EOF => return Ok(()),
            }
            self.pc += 1;
        }
    }
}
