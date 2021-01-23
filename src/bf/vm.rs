use super::common::Instruction;
use Instruction::*;

use std::io::{Read, Write};

use std::result::Result;

pub struct Interpreter<'a, T: Read, U: Write> {
    read_stream: &'a mut T,
    write_stream: &'a mut U,
    instructions: &'a [Instruction],
    pc: usize,
    memory: Vec<u8>,
    memory_size: usize,
    mem_idx: usize,
}

impl<'a, T: Read, U: Write> Interpreter<'a, T, U> {
    pub fn new(src: &'a [Instruction], input: &'a mut T, output: &'a mut U) -> Self {
        Self::with_capacity(src, input, output, 30000)
    }

    pub fn with_capacity(
        instructions: &'a [Instruction],
        input: &'a mut T,
        output: &'a mut U,
        capacity: usize,
    ) -> Self {
        Self {
            read_stream: input,
            write_stream: output,
            instructions,
            pc: 0,
            memory: vec![0; capacity],
            memory_size: capacity,
            mem_idx: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        loop {
            let instruction = &self.instructions[self.pc];
            match *instruction {
                Add(amt) => self.memory[self.mem_idx] = self.memory[self.mem_idx].wrapping_add(amt),
                Jez(dst) => {
                    if self.memory[self.mem_idx] == 0 {
                        self.pc += dst as usize
                    }
                }
                Jnz(dst) => {
                    if self.memory[self.mem_idx] != 0 {
                        self.pc -= dst as usize
                    }
                }
                Right(amt) => self.mem_idx = (self.mem_idx + amt as usize) % self.memory_size,
                Left(amt) => self.mem_idx = (self.mem_idx - amt as usize) % self.memory_size,
                Write => match self
                    .write_stream
                    .write(&self.memory[self.mem_idx..self.mem_idx + 1])
                {
                    Ok(_) => {}
                    Err(_) => return Err("Failed to write to output."),
                },
                Read => match self
                    .read_stream
                    .read(&mut self.memory[self.mem_idx..self.mem_idx + 1])
                {
                    Ok(_) => {}
                    Err(_) => return Err("Failed to read from input."),
                },

                Halt => return Ok(()),
            }
            self.pc += 1;
        }
    }
}
