use nom::IResult;
use nom::bytes::complete::tag;
use crate::assembler::Token;
use crate::assembler::opcode_parsers::*;
use crate::assembler::operand_parsers::integer_operand;
use crate::assembler::register_parsers::register;
use crate::instructions::Opcode;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        match self.opcode {
            Token::Op { code } => match code {
                _ => {
                    results.push(code as u8);
                }
            },
            _ => {
                println!("Non-opcode found in opcode field");
                std::process::exit(1);
            }
        };

        for operand in vec![&self.operand1, &self.operand2, &self.operand3] {
            match operand {
                Some(t) => AssemblerInstruction::extract_operand(t, &mut results),
                None => {}
            }
        }

        return results;
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
        match t {
            Token::Register { reg_num } => {
                results.push(*reg_num);
            }
            Token::IntegerOperand { value } => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        };
    }
}

pub fn instruction(s: &str) -> IResult<&str, AssemblerInstruction> {
    if s == "" {
        return tag(" ")("").map(|(res, _)| (res, AssemblerInstruction {opcode: Token::Op{code: Opcode::IGL}, operand1: None, operand2: None, operand3: None}));
    };

    const O: [&str; 1] = ["hlt"];
    const OR: [&str; 3] = ["jmp", "jmpf", "jmpb"];
    const ORI: [&str; 1] = ["load"];
    const ORR: [&str; 1] = ["jeq"];
    const ORRR: [&str; 10] = ["add", "sub", "mul", "div", "eq", "neq", "gte", "gt", "lte", "lt"];

    let mut opc: Token = Token::Op{code: Opcode::IGL};
    let mut opr1: Option<Token> = None;
    let mut opr2: Option<Token> = None;
    let mut opr3: Option<Token> = None;

    let sv: Vec<&str> = s.trim().split(' ').collect();
    if O.iter().any( |&i| i==sv[0] ) {
        opc = opcode(s);
    } else if OR.iter().any( |&i| i==sv[0] ) {
        opc = opcode(sv[0]);
        opr1 = Some(register(sv[1])?.1);
    } else if ORI.iter().any( |&i| i==sv[0] ) {
        opc = opcode(sv[0]);
        opr1 = Some(register(sv[1])?.1);
        opr2 = Some(integer_operand(sv[2])?.1);
    } else if ORR.iter().any( |&i| i==sv[0] ) {
        opc = opcode(sv[0]);
        opr1 = Some(register(sv[1])?.1);
        opr2 = Some(register(sv[2])?.1);
    } else if ORRR.iter().any( |&i| i==sv[0] ) {
        opc = opcode(sv[0]);
        opr1 = Some(register(sv[1])?.1);
        opr2 = Some(register(sv[2])?.1);
        opr3 = Some(register(sv[3])?.1);
    };

    Ok((
        "",
        AssemblerInstruction {
            opcode: opc,
            operand1: opr1,
            operand2: opr2,
            operand3: opr3,
        }
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::Opcode;

    #[test]
    fn test_parse_instruction_one() {
        let result = instruction("hlt");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::HLT },
                    operand1: None,
                    operand2: None,
                    operand3: None
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_three() {
        let result = instruction("load $0 #100");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::LOAD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::IntegerOperand { value: 100 }),
                    operand3: None
                }
            ))
        );
    }
}