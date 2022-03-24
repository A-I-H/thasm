pub mod opcode_parsers;

use crate::instructions::Opcode;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op{code: Opcode},
    Register{reg_num: u8},
}

impl From<&str> for Token {
    fn from(s: &str) -> Self {
        match s {
            "load" => Token::Op{code: Opcode::LOAD},
            _ => unimplemented!("Nope"),
        }
    }
}