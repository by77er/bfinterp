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
        let mut total: usize = 0;
        loop {
            let instruction = &self.instructions[self.pc];
            // println!("inst:{:?}, pc:{}, ptr:{}", instruction, self.pc, self.mem_idx);
            match *instruction {
                ZeroRight(n) => {
                    let target = (self.mem_idx + n as usize) % self.memory_size;
                    while self.mem_idx != target {
                        self.memory[self.mem_idx] = 0;
                        self.mem_idx = (self.mem_idx + 1 as usize) % self.memory_size;
                    }
                },
                ZeroLeft(n) => {
                    let target = (self.mem_idx - n as usize) % self.memory_size;
                    while self.mem_idx != target {
                        self.memory[self.mem_idx] = 0;
                        self.mem_idx = (self.mem_idx - 1 as usize) % self.memory_size;
                    }
                },
                AddMoveRight(n) => {
                    let new_idx = (self.mem_idx + n as usize) % self.memory_size;
                    self.memory[new_idx] = self.memory[new_idx].wrapping_add(self.memory[self.mem_idx]);
                    self.memory[self.mem_idx] = 0;
                },
                AddMoveLeft(n) => {
                    let new_idx = (self.mem_idx - n as usize) % self.memory_size;
                    self.memory[new_idx] = self.memory[new_idx].wrapping_add(self.memory[self.mem_idx]);
                    self.memory[self.mem_idx] = 0;

                },
                SearchLeft(num) => {
                    while self.memory[self.mem_idx] != 0 {
                        self.mem_idx = (self.mem_idx - num as usize) % self.memory_size
                    }
                },
                SearchRight(num) => {
                    while self.memory[self.mem_idx] != 0 {
                        self.mem_idx = (self.mem_idx + num as usize) % self.memory_size
                    }
                }
                Zero => self.memory[self.mem_idx] = 0,
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
                Halt => {
                    println!("{} instructions executed.", total);
                    // println!("{:?}", &self.memory[0..30]);
                    return Ok(())
                },
            }
            total += 1;
            self.pc += 1;
        }
    }
}
