pub mod opcode_parsers;
pub mod register_parsers;

use nom::{number, combinator::complete};

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
            _ => {
                // Token::Register{ reg_num: s.parse::<u8>().unwrap() }\
                match s.parse::<u8>() {
                    Ok(v) => Token::Register{ reg_num: v },
                    Err(_) => unimplemented!("Nope"),
                }
            },
        }
    }
}