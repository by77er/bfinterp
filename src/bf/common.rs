#[derive(Debug)]
pub enum Instruction {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Input,
    LeftLoop,
    RightLoop,
    EOF
}