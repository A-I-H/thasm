use nom::{
    bytes::complete::tag_no_case, character::complete::digit1, combinator::map_res, IResult,
};

use super::Token;
pub fn register(s: &str) -> IResult<&str, Token> {
    let t = tag_no_case("$")(s)?.0;
    map_res(digit1, str::parse::<u8>)(t).map(|(res, num)| (res, Token::Register { reg_num: num }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = register("$0");
        assert!(result.is_ok());
        let result = register("0");
        assert!(result.is_err());
        let result = register("$a");
        assert!(result.is_err());
    }
}
