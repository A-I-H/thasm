use nom::{IResult, multi::many1};

use crate::assembler::instruction_parsers::{AssemblerInstruction, instruction_one};

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>
}

// named!(pub program<CompleteStr, Program>,
//     do_parse!(
//         instructions: many1!(instruction_one) >>
//         (
//             Program {
//                 instructions: instructions
//             }
//         )
//     )
// );

pub fn program(s: &str) -> IResult<&str, Program> {
    let t = many1(instruction_one(s));
}