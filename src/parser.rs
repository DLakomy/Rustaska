use std::{error::Error, fmt::Display};

use super::model::*;

#[derive(Debug)]
pub struct ParseError {
    ctx: String,
    msg: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error when parsing \"{}...\":\n{}", self.ctx, self.msg)
    }
}

impl Error for ParseError {}

pub fn parse_record(input: &str) -> Result<Record, ParseError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    impl ParseError {
        pub fn new(ctx: String, msg: String) -> Self {
            ParseError { ctx, msg }
        }
    }

    #[test]
    fn test_parse_record() -> Result<(), ParseError> {
        let input = "\
Record: 12
P01: 321
P02: \"sample text\"
P03:  -66
P04:  \"sth\"
%\n";

        let fields = vec![
            Field {
                id: 1,
                value: FieldVal::Num(321),
            },
            Field {
                id: 2,
                value: FieldVal::Str("sample text".to_owned()),
            },
            Field {
                id: 3,
                value: FieldVal::Num(-66),
            },
            Field {
                id: 4,
                value: FieldVal::Str("sth".to_owned()),
            },
        ];
        let expected = Record { id: 12, fields };

        let result = parse_record(input)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_parse_error_display() {
        let err = ParseError::new("Record 1".to_owned(), "whatever".to_owned());
        let obtained = err.to_string();
        let expected = "\
Error when parsing \"Record 1...\":
whatever"
            .to_owned();

        assert_eq!(obtained, expected)
    }
}
