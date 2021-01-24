use super::common::Instruction;
use Instruction::*;

use std::io::{Read, Write};

use std::result::Result;

use std::collections::HashMap;

#[derive(Default, Debug)]
struct Profile {
    add: usize,
    jez: usize,
    jnz: usize,
    left: usize,
    right: usize,
    search_left: usize,
    search_right: usize,
    add_move_left: usize,
    add_move_right: usize,
    zero_right: usize,
    zero_left: usize,
    zero: usize,
    write: usize,
    read: usize,
    halt: usize 
}

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
        // let mut profile: Profile = Default::default();
        // let mut total: usize = 0;
        // let mut subroutines: HashMap<&'a[Instruction], usize> = HashMap::new();
        loop {
            let instruction = &self.instructions[self.pc];
            // println!("inst:{:?}, pc:{}, ptr:{}", instruction, self.pc, self.mem_idx);
            match *instruction {
                Mandel(x, n) => {
                    while self.memory[self.mem_idx] != 0 {
                        self.memory[self.mem_idx + x as usize + n as usize] = self.memory[self.mem_idx + x as usize + n as usize].wrapping_add(self.memory[self.mem_idx + x as usize]);
                        self.memory[self.mem_idx + x as usize] = 0;
                        self.mem_idx = (self.mem_idx - n as usize) % self.memory_size;
                    }
                },
                ZeroRight(n) => {
                    // profile.zero_right += 1;
                    let target = (self.mem_idx + n as usize) % self.memory_size;
                    while self.mem_idx != target {
                        self.memory[self.mem_idx] = 0;
                        self.mem_idx = (self.mem_idx + 1 as usize) % self.memory_size;
                    }
                },
                ZeroLeft(n) => {
                    // profile.zero_left += 1;
                    let target = (self.mem_idx - n as usize) % self.memory_size;
                    while self.mem_idx != target {
                        self.memory[self.mem_idx] = 0;
                        self.mem_idx = (self.mem_idx - 1 as usize) % self.memory_size;
                    }
                },
                AddMoveRight(n) => {
                    // profile.add_move_right += 1;
                    let new_idx = (self.mem_idx + n as usize) % self.memory_size;
                    self.memory[new_idx] = self.memory[new_idx].wrapping_add(self.memory[self.mem_idx]);
                    self.memory[self.mem_idx] = 0;
                },
                AddMoveLeft(n) => {
                    // profile.add_move_left += 1;
                    let new_idx = (self.mem_idx - n as usize) % self.memory_size;
                    self.memory[new_idx] = self.memory[new_idx].wrapping_add(self.memory[self.mem_idx]);
                    self.memory[self.mem_idx] = 0;

                },
                SearchLeft(num) => {
                    // profile.search_left += 1;
                    while self.memory[self.mem_idx] != 0 {
                        self.mem_idx = (self.mem_idx - num as usize) % self.memory_size
                    }
                },
                SearchRight(num) => {
                    // profile.search_right += 1;
                    while self.memory[self.mem_idx] != 0 {
                        self.mem_idx = (self.mem_idx + num as usize) % self.memory_size
                    }
                }
                Zero => {
                    // profile.zero += 1;
                    self.memory[self.mem_idx] = 0
                },
                Add(amt) => {
                    // profile.add += 1;
                    self.memory[self.mem_idx] = self.memory[self.mem_idx].wrapping_add(amt)
                },
                Jez(dst) => {
                    // profile.jez += 1;
                    if self.memory[self.mem_idx] == 0 {
                        self.pc += dst as usize
                    }
                }
                Jnz(dst) => {
                    // *subroutines.entry(&self.instructions[self.pc - dst as usize..=self.pc]).or_insert(0) += 1;
                    // profile.jnz += 1;
                    if self.memory[self.mem_idx] != 0 {
                        self.pc -= dst as usize
                    }
                }
                Right(amt) => {
                    // profile.right += 1;
                    self.mem_idx = (self.mem_idx + amt as usize) % self.memory_size
                },
                Left(amt) => {
                    // profile.left += 1;
                    self.mem_idx = (self.mem_idx - amt as usize) % self.memory_size
                },
                Write => match self
                    .write_stream
                    .write(&self.memory[self.mem_idx..self.mem_idx + 1])
                {
                    Ok(_) => {
                        // profile.write += 1
                    }
                    Err(_) => return Err("Failed to write to output."),
                },
                Read => match self
                    .read_stream
                    .read(&mut self.memory[self.mem_idx..self.mem_idx + 1])
                {
                    Ok(_) => {
                        // profile.read += 1
                    }
                    Err(_) => return Err("Failed to read from input."),
                },
                Halt => {
                    /*
                    profile.halt += 1;
                    println!("{:#?}", profile);
                    let mut loops = subroutines.into_iter().collect::<Vec<(&[Instruction], usize)>>();
                    loops.sort_by(|x, y| x.1.cmp(&y.1));
                    for e in loops {
                        println!("{} -> {:?}", e.1, e.0);
                    }
                    println!("{} instructions executed.", total);
                    println!("{:?}", &self.memory[0..30]);
                    */
                    return Ok(())
                },
            }
            // total += 1;
            self.pc += 1;
        }
    }
}
