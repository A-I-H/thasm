use crate::instructions::Opcode;
use super::Token;

pub fn opcode(s: &str) -> Token {
    Token::Op{code: Opcode::from(s)}
}

#[cfg(test)]
mod tests {
    use crate::{assembler::Token, instructions::Opcode};

    use super::*;

    #[test]
    fn test_opcode_load() {
        // First tests that the opcode is detected and parsed correctly
        let token = opcode("load");
        assert_eq!(token, Token::Op{code: Opcode::LOAD});

        // Tests that an invalid opcode isn't recognized
        let token = opcode("aold");
        assert_eq!(token, Token::Op{code: Opcode::IGL});
    }
}