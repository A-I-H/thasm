use std::num::ParseIntError;

use nom::{bytes::complete::tag, IResult, number, combinator::complete, error::VerboseError};

use super::Token;

pub fn register(s: &str) -> IResult<&str, Token> {
    tag("$")(s.trim())
        .map(|(num, res)| {
            match num.parse::<u8>() {
                Ok(n) => (res, Token::Register{ reg_num: n }),
                Err(_) => nom::Err,
            }
        })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = register("$0");
        assert_eq!(result, Token::Register{reg_num: 0})
        // assert_eq!(result.is_ok(), true);
        // let result = register("0");
        // assert_eq!(result.is_ok(), false);
        // let result = register("$a");
        // assert_eq!(result.is_ok(), false);
  }

}