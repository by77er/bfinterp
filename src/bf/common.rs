/// Tokens straight from the input stream
#[derive(Debug)]
pub enum Token {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    RightLoop,
    LeftLoop,
    Output,
    Input,
    EOF,
}

/// Intermediate representation
#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Loop(Vec<Node>),
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Input,
    Halt,
}

/// Instructions to be executed by the VM
#[derive(Debug)]
pub enum Instruction {
    Add(u8),  // Subtraction isn't required
    Jez(u16), // Jumps forward
    Jnz(u16), // Jumps backward
    Left(u16),
    Right(u16),
    Write,
    Read,
    Halt,
}
