pub mod opcode_parsers;
pub mod register_parsers;
pub mod operand_parsers;
pub mod instruction_parsers;
pub mod program_parsers;

use crate::instructions::Opcode;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op{code: Opcode},
    Register{reg_num: u8},
    IntegerOperand{value: i32},
}

impl From<&str> for Token {
    fn from(s: &str) -> Self {
        match &s.to_lowercase() as &str {
            "load" => Token::Op{code: Opcode::LOAD},
            _ => unimplemented!(),
        }
    }
}