use nom::{bytes::complete::tag, IResult};

use crate::instructions::Opcode;

use super::Token;

fn opcode_load(s: &str) -> Token {
    // tag("load") >> (Token::Op{code: Opcode::LOAD})
    // let token: Token;
    // if tag("load")(s).is_ok() {
    //     token = Token::Op{code: Opcode::LOAD};
    // };
    // return token;

    // tag("load")(s).unwrap().1

    // match tag("load")(s) {
    //     Ok(_v) => return Token::Op{code: Opcode::LOAD},
    //     Err(_e) => return Token::Op{code: Opcode::IGL},
    // }
}

#[cfg(test)]
mod tests {
    use crate::assembler::Token;

    use super::*;

    #[test]
    fn test_opcode_load() {
        // First tests that the opcode is detected and parsed correctly
        let result = opcode_load("load");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op{code: Opcode::LOAD});
        assert_eq!(rest, "");

        // Tests that an invalid opcode isn't recognized
        let result = opcode_load("aold");
        assert_eq!(result.is_ok(), false);
    }
}