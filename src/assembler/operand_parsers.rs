use nom::{IResult, bytes::complete::tag_no_case, character::complete::digit1, combinator::map_res};

use super::Token;

pub fn integer_operand(s: &str) -> IResult<&str, Token> {
    let t = tag_no_case("#")(s)?.0;
    map_res(digit1, str::parse::<i32>)(t)
        .map(|(res, num)| (res, Token::IntegerOperand{value: num}))

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer_operand() {
        // Test a valid integer operand
        let result = integer_operand("#10");
        assert_eq!(result.is_ok(), true);
        let (rest, value) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(value, Token::IntegerOperand{value: 10});

        // Test an invalid one (missing the #)
        let result = integer_operand("10");
        assert_eq!(result.is_ok(), false);
    }
}