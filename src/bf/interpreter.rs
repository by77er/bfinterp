use super::common::Instruction;
use Instruction::*;

use std::io::{Read, Write};
use std::iter::Iterator;

use std::result::Result;

pub struct BFInterpreter<'a, T: Read, U: Write, V: Iterator<Item=Instruction>> {
    read_stream: &'a mut T,
    write_stream: &'a mut U,
    instruction_source: V,
    instruction_buffer: Vec<Instruction>,
    memory: Vec<u8>,
    mem_idx: usize,
    stack: Vec<usize>,
    pc: usize
}

impl<'a, T: Read, U: Write, V: Iterator<Item=Instruction>> BFInterpreter<'a, T, U, V> {
    pub fn new(src: V, input: &'a mut T, output: &'a mut U) -> Self {
        Self::with_capacity(src, input, output, 30000)
    }

    pub fn with_capacity(src: V, input: &'a mut T, output: &'a mut U, capacity: usize) -> Self {
        Self {
            read_stream: input,
            write_stream: output,
            instruction_source: src,
            instruction_buffer: Vec::new(),
            memory: vec![0; capacity],
            mem_idx: 0,
            stack: Vec::new(),
            pc: 0
        }
    }

    #[inline]
    fn read_instructions(&mut self, n: usize) -> Result<usize, ()> {
        for _ in 0..n {
            if let Some(i) = self.instruction_source.next() {
                self.instruction_buffer.push(i)
            } else {
                return Err(())
            }
        }
        Ok(n)
    }

    fn get_instruction(&mut self, idx: usize) -> Result<&Instruction, ()> {
        if self.instruction_buffer.len() < (idx + 1) {
            match self.read_instructions((idx + 1) - self.instruction_buffer.len()) {
                Ok(_) => {},
                Err(()) => return Err(())
            }
        }
        Ok(&self.instruction_buffer[idx])

    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        loop {
            let ins = match self.get_instruction(self.pc) {
                Ok(i) => i,
                Err(_) => return Err("Failed to read instruction.")
            };
            match ins {
                MoveRight => {
                    if self.mem_idx == self.memory.len() - 1 {
                        self.mem_idx = 0;
                    } else {
                        self.mem_idx += 1;
                    }
                },
                MoveLeft => {
                    if self.mem_idx == 0 {
                        self.mem_idx = self.memory.len() - 1;
                    } else {
                        self.mem_idx -= 1;
                    }
                },
                Increment => {
                    self.memory[self.mem_idx] = self.memory[self.mem_idx].wrapping_add(1);
                },
                Decrement => {
                    self.memory[self.mem_idx] = self.memory[self.mem_idx].wrapping_sub(1);
                },
                Output => {
                    match self.write_stream.write(&self.memory[self.mem_idx..self.mem_idx+1]) {
                        Ok(_) => {},
                        Err(_) => return Err("Failed to write to output.")
                    }
                },
                Input => {
                    match self.read_stream.read(&mut self.memory[self.mem_idx..self.mem_idx+1]) {
                        Ok(_) => {},
                        Err(_) => return Err("Failed to read from input.")
                    }
                },
                LeftLoop => {
                    if self.memory[self.mem_idx] == 0 { // jump forward
                        let mut count = 1;
                        while count != 0 {
                            self.pc += 1;
                            let stru = match self.get_instruction(self.pc) {
                                Ok(i) => i,
                                Err(_) => return Err("Failed to read instruction.")
                            };
                            match stru {
                                EOF => return Err("Reached EOF while searching for ]. Unmatched [."),
                                LeftLoop => count += 1,
                                RightLoop => count -= 1,
                                _ => continue
                            }
                        }
                    } else { // continue and push to the stack
                        self.stack.push(self.pc);
                    }
                },
                RightLoop => {
                    // Establish that stack has an element
                    if self.stack.len() == 0 {
                        return Err("Stack is unexpectedly empty. Unmatched ].");
                    }
                    if self.memory[self.mem_idx] != 0 { // jump backward
                        self.pc = self.stack[self.stack.len() - 1];
                    } else { // continue and pop the stack
                        self.stack.pop().unwrap();
                    }
                },
                EOF => return Ok(()),
            }
            self.pc += 1;
        }
    }
}