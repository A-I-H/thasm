use nom::{bytes::complete::tag_no_case, IResult, combinator::map_res, character::complete::digit1};

use super::Token;
pub fn register(s: &str) -> IResult<&str, Token> {
    let t = tag_no_case("$")(s)?.0;
    map_res(digit1, str::parse::<u8>)(t)
        .map(|(res, num)| (res, Token::Register{reg_num: num}))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = register("$0");
        assert_eq!(result.is_ok(), true);
        let result = register("0");
        assert_eq!(result.is_ok(), false);
        let result = register("$a");
        assert_eq!(result.is_ok(), false);
  }

}