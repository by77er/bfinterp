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
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Instruction {
    Add(u8),  // Subtraction isn't required
    Jez(u16), // Jumps forward
    Jnz(u16), // Jumps backward
    Left(u16),
    Right(u16),
    SearchLeft(u16),
    SearchRight(u16),
    AddMoveRight(u16),
    AddMoveLeft(u16),
    ZeroRight(u16),
    ZeroLeft(u16),
    Mandel(u16, u16),
    Zero, // Zero current cell
    Write,
    Read,
    Halt,
}
