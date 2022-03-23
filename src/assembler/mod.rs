pub mod opcode_parsers;

use crate::instructions::Opcode;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op{code: Opcode},
}