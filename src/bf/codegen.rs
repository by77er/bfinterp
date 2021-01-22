// Takes Nodes and produces Instructions

use super::common::{Node, Instruction};

pub fn generate_code(mut nodes: Vec<Node>) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    while let Some(node) = nodes.pop() {
        match node {
            Node::Loop(inner_nodes) => {
                let mut inner_instructions = generate_code(inner_nodes);
                let inner_size = inner_instructions.len();
                instructions.push(Instruction::Jnz(inner_size as u16 + 1));
                instructions.append(&mut inner_instructions);
                instructions.push(Instruction::Jez(inner_size as u16 + 1));
            }
            Node::MoveRight         => instructions.push(Instruction::Shift(optimize_ptr(1, &mut nodes))),
            Node::MoveLeft          => instructions.push(Instruction::Shift(optimize_ptr(-1, &mut nodes))),
            Node::Increment         => instructions.push(Instruction::Add(optimize_math(1, &mut nodes))),
            Node::Decrement         => instructions.push(Instruction::Add(optimize_math(255, &mut nodes))),
            Node::Output            => instructions.push(Instruction::Write),
            Node::Input             => instructions.push(Instruction::Read),
            Node::Halt              => instructions.push(Instruction::Halt)
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
            },
            Some(Node::Decrement) => {
                acc = acc.wrapping_sub(1);
                nodes.pop();
            },
            _ => return acc
        }
    }
}

fn optimize_ptr(start: i16, nodes: &mut Vec<Node>) -> i16 {
    let mut acc = start;
    loop {
        match nodes.last() {
            Some(Node::MoveRight) => {
                acc += 1;
                nodes.pop();
            },
            Some(Node::MoveLeft) => {
                acc -= 1;
                nodes.pop();
            },
            _ => return acc
        }
    }
}