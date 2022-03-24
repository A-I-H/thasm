use nom::{bytes::complete::tag, IResult, sequence::delimited};

use super::Token;

fn opcode_load(s: &str) -> IResult<&str, Token> {
    tag("load")(s)
        .map(|(res,input)| (res, input.into()))
}

#[cfg(test)]
mod tests {
    use crate::{assembler::Token, instructions::Opcode};

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