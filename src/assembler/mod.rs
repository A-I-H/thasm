pub mod opcode_parsers;

use crate::instructions::Opcode;

#[derive(Debug, PartialEq)]
pub enum Token {
    Incorrect,
    Op{code: Opcode},
}

impl From<&str> for Token {
    fn from(s: &str) -> Self {
        match s {
            "load" => Token::Op{code: Opcode::LOAD},
            _ => Token::Incorrect,
        }
    }
}