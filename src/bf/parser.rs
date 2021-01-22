// Takes tokens and converts to Nodes in a tree

use super::common::{Token, Node};

use std::iter::Iterator;

pub struct Parser<T: Iterator<Item=Token>> {
    tokens: T,
    eof: bool
}

impl<T: Iterator<Item=Token>> Parser<T> {
    pub fn new(tokens: T) -> Self {
        Self {
            tokens,
            eof: false
        }
    }

    fn get_node(&mut self) -> Option<Node> {
        if let Some(t) = self.tokens.next() {
            return Some(match t {
                Token::MoveRight => Node::MoveRight,
                Token::MoveLeft => Node::MoveLeft,
                Token::Increment => Node::Increment,
                Token::Decrement => Node::Decrement,
                Token::Output => Node::Output,
                Token::Input => Node::Input,
                Token::EOF => {
                    self.eof = true;
                    Node::Halt
                },
                Token::LeftLoop => {
                    let mut v = Vec::new();
                    while let Some(n) = self.get_node() {
                        if n == Node::Halt {
                            panic!("Error: Loop syntax error")
                        }
                        v.push(n);
                    }
                    Node::Loop(v)
                }
                Token::RightLoop => {
                    return None
                }
            })
        } else if !self.eof {
            panic!("Error: Iterator ended before EOF");
        } else {
            return None
        }
    }
}

impl<T: Iterator<Item=Token>> Iterator for Parser<T> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_node()
    }
}