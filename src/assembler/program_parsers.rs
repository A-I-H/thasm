use nom::{multi::many1, IResult};

use crate::assembler::instruction_parsers::{instruction, AssemblerInstruction};

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut program = vec![];
        for instruction in &self.instructions {
            program.append(&mut instruction.to_bytes());
        }
        program
    }
}

pub fn program(s: &str) -> IResult<&str, Program> {
    many1(instruction)(s).map(|(res, instructions)| (res, Program { instructions }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program() {
        let result = program("load $0 #10");
        assert!(result.is_ok());
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(1, p.instructions.len());
        // TODO: Figure out an ergonomic way to test the AssemblerInstruction returned
    }

    #[test]
    fn test_parse_multiple_instructions() {
        let result = program("load $0 #10 load $1 #5");
        assert!(result.is_ok());
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(2, p.instructions.len());
        println!("{:?}", p.instructions);

        let result = program("load $0 #10 load $1 #5 add $0 $1 $2");
        assert!(result.is_ok());
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(3, p.instructions.len());
        println!("{:?}", p.instructions);
    }

    #[test]
    fn test_program_to_bytes() {
        let result = program("load $0 #100");
        assert!(result.is_ok());
        let (_, program) = result.unwrap();
        let bytecode = program.to_bytes();
        assert_eq!(bytecode.len(), 4);
    }
}
