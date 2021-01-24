// Takes Nodes and produces Instructions

use super::common::{Instruction, Node};

pub fn generate_code(nodes: Vec<Node>) -> Vec<Instruction> {
    let code = generate_raw_code(nodes);
    // Optimize at instruction level
    let code = optimize_instruction_chain(&code);
    code
}

// Instruction opt
fn optimize_instruction_chain(instrs: &[Instruction]) -> Vec<Instruction> {
    let mut new_instructions = Vec::new();
    new_instructions.extend(instrs);
    new_instructions.reverse();
    zero(&mut new_instructions);
    dedup_zero(&mut new_instructions);
    search(&mut new_instructions);
    move_cell(&mut new_instructions);
    zero_area(&mut new_instructions);
    zero_area(&mut new_instructions);
    mandel(&mut new_instructions);
    new_instructions.reverse();
    new_instructions
}

// [+] and [-] forms
fn zero(instrs: &mut Vec<Instruction>) {
    let mut idx = 0;
    while idx + 3 <= instrs.len() {
        match instrs[idx..idx+3] {
            [Instruction::Jez(2), Instruction::Add(1), Instruction::Jnz(2)] => {
                instrs.splice(idx..idx+3, [Instruction::Zero].iter().cloned());
            },
            [Instruction::Jez(2), Instruction::Add(255), Instruction::Jnz(2)] => {
                instrs.splice(idx..idx+3, [Instruction::Zero].iter().cloned());
            },
            [Instruction::Jez(2), Instruction::Zero, Instruction::Jnz(2)] => {
                instrs.splice(idx..idx+3, [Instruction::Zero].iter().cloned());
            },
            _ => {
                idx += 1;
            }
        }
    }
}

// Zero Zero -> Zero
fn dedup_zero(instrs: &mut Vec<Instruction>) {
    let mut idx = 0;
    while idx + 2 <= instrs.len() {
        match instrs[idx..idx+2] {
            [Instruction::Zero, Instruction::Zero] => {
                instrs.splice(idx..idx+2, [Instruction::Zero].iter().cloned());
            },
            _ => {
                idx += 1;
            }
        }
    }
}

// [>] and [<] forms
fn search(instrs: &mut Vec<Instruction>) {
    let mut idx = 0;
    while idx + 3 <= instrs.len() {
        match instrs[idx..idx+3] {
            [Instruction::Jez(2), Instruction::Left(n), Instruction::Jnz(2)] => {
                instrs.splice(idx..idx+3, [Instruction::SearchLeft(n)].iter().cloned());
            },
            [Instruction::Jez(2), Instruction::Right(n), Instruction::Jnz(2)] => {
                instrs.splice(idx..idx+3, [Instruction::SearchRight(n)].iter().cloned());
            },
            _ => {
                idx += 1;
            }
        }
    }
}

// [-<<<<<<<<<<+>>>>>>>>>>] form
fn move_cell(instrs: &mut Vec<Instruction>) {
    let mut idx = 0;
    while idx + 6 <= instrs.len() {
        match instrs[idx..idx+6] {
            [Instruction::Jez(_), Instruction::Add(n), Instruction::Right(r), Instruction::Add(n2), Instruction::Left(l), Instruction::Jnz(_)] => {
                if r == l && n == 255 && n2 == 1 {
                    // println!("{:?}", instrs);
                    instrs.splice(idx..idx+6, [Instruction::AddMoveRight(r)].iter().cloned());
                } else {
                    idx += 1;
                }
            },
            [Instruction::Jez(_), Instruction::Add(n), Instruction::Left(l), Instruction::Add(n2), Instruction::Right(r), Instruction::Jnz(_)] => {
                if r == l && n == 255 && n2 == 1 {
                    // println!("{:?}", instrs);
                    instrs.splice(idx..idx+6, [Instruction::AddMoveLeft(l)].iter().cloned());
                } else {
                    idx += 1;
                }
            },
            _ => {
                idx += 1;
            }
        }
    }
}

// [-]>[-]>[-] and <[-]<[-]<[-] form
fn zero_area(instrs: &mut Vec<Instruction>) {
    let mut idx = 0;
    while idx + 2 <= instrs.len() {
        match instrs[idx..idx+2] {
            [Instruction::Zero, Instruction::Right(1)] => {
                instrs.splice(idx..idx+2, [Instruction::ZeroRight(1)].iter().cloned());
            },
            [Instruction::ZeroRight(n), Instruction::ZeroRight(n2)] => {
                instrs.splice(idx..idx+2, [Instruction::ZeroRight(n+n2)].iter().cloned());
            },
            [Instruction::Zero, Instruction::Left(1)] => {
                instrs.splice(idx..idx+2, [Instruction::ZeroLeft(1)].iter().cloned());
            },
            [Instruction::ZeroLeft(n), Instruction::ZeroLeft(n2)] => {
                instrs.splice(idx..idx+2, [Instruction::ZeroLeft(n+n2)].iter().cloned());
            },
            _ => {
                idx += 1;
            }
        }
    }
}

// very common sequence in mandelbrot
// while the current cell isn't 0:
//   addmove the value of the cell x to the left to the cell x + n to the left, then change cell to current - n
fn mandel(instrs: &mut Vec<Instruction>) {
    let mut idx = 0;
    while idx + 5 <= instrs.len() {
        match instrs[idx..idx+5] {
            [Instruction::Jez(4), Instruction::Right(x), Instruction::AddMoveRight(n),  Instruction::Left(q), Instruction::Jnz(4)] => {
                if x + n == q {
                    instrs.splice(idx..idx+5, [Instruction::Mandel(x, n)].iter().cloned());
                } else {
                    idx += 1;
                }
            },
            _ => {
                idx += 1;
            }
        }
    }
}


// Basic codegen

fn generate_raw_code(mut nodes: Vec<Node>) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    while let Some(node) = nodes.pop() {
        match node {
            Node::Loop(inner_nodes) => {
                let inner_raw_instructions = generate_raw_code(inner_nodes);
                let inner_instructions = optimize_instruction_chain(&inner_raw_instructions);
                let inner_size = inner_instructions.len();
                instructions.push(Instruction::Jnz(inner_size as u16 + 1));
                instructions.extend(&inner_instructions);
                instructions.push(Instruction::Jez(inner_size as u16 + 1));
            }
            Node::MoveRight => instructions.push(optimize_ptr(1, &mut nodes)),
            Node::MoveLeft => instructions.push(optimize_ptr(-1, &mut nodes)),
            Node::Increment => instructions.push(Instruction::Add(optimize_math(1, &mut nodes))),
            Node::Decrement => instructions.push(Instruction::Add(optimize_math(255, &mut nodes))),
            Node::Output => instructions.push(Instruction::Write),
            Node::Input => instructions.push(Instruction::Read),
            Node::Halt => instructions.push(Instruction::Halt),
        }
    }
    instructions
}

fn optimize_math(start: u8, nodes: &mut Vec<Node>) -> u8 {
    let mut acc = start;
    loop {
        match nodes.last() {
            Some(Node::Increment) => {
                acc = acc.wrapping_add(1);
                nodes.pop();
            }
            Some(Node::Decrement) => {
                acc = acc.wrapping_sub(1);
                nodes.pop();
            }
            _ => return acc,
        }
    }
}

fn optimize_ptr(start: i16, nodes: &mut Vec<Node>) -> Instruction {
    let mut acc = start;
    loop {
        match nodes.last() {
            Some(Node::MoveRight) => {
                acc += 1;
                nodes.pop();
            }
            Some(Node::MoveLeft) => {
                acc -= 1;
                nodes.pop();
            }
            _ => {
                if acc > 0 {
                    return Instruction::Right(acc as u16);
                } else {
                    return Instruction::Left(acc.abs() as u16);
                }
            }
        }
    }
}
