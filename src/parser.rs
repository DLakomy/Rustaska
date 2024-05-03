mod lexer;

use logos::{Lexer, Logos};

use self::lexer::Token;

use super::model::*;
use std::{error::Error, fmt::Display};

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

// disclaimer: I'm focused on the happy path only, not on good error handling
pub fn parse_record(input: &str) -> Result<Record, ParseError> {
    let ctx = "header".to_owned();
    let mut lexer = Token::lexer(input);
    match lexer.next() {
        Some(Ok(Token::RecordStart)) => match lexer.next() {
            Some(Ok(Token::Number(id))) => {
                if lexer.next() != Some(Ok(Token::NewLine)) {
                    return Err(ParseError {
                        ctx,
                        msg: "Expected a new line".to_owned(),
                    });
                }
                let fields = parse_fields(&mut lexer)?;
                Ok(Record { id, fields })
            }
            _ => Err(ParseError {
                ctx,
                msg: "Expected a record id (a number)".to_owned(),
            }),
        },
        _ => Err(ParseError {
            ctx,
            msg: "Expected string: \"Record:\"".to_owned(),
        }),
    }
}

fn parse_fields<'src>(
    lexer: &mut Lexer<'src, Token<'src>>,
) -> Result<Vec<Field<'src>>, ParseError> {
    let mut fields = Vec::new();

    while let Some(Ok(token)) = lexer.next() {
        match token {
            Token::FieldId(id) => {
                let field = parse_field(id, lexer)?;
                fields.push(field)
            }
            Token::RecordEnd => break,
            _ => {
                return Err(ParseError {
                    ctx: "fields".to_owned(),
                    msg: "Expected field id (eg. \"P12:\")".to_owned(),
                })
            }
        }
    }

    if lexer.remainder() != "\n" {
        return Err(ParseError {
            ctx: "fields".to_owned(),
            msg: "Unexpected content after RecordEnd".to_owned(),
        });
    };

    Ok(fields)
}

fn parse_field<'src>(
    id: i32,
    lexer: &mut Lexer<'src, Token<'src>>,
) -> Result<Field<'src>, ParseError> {
    let ctx = "field";

    let value: FieldVal = {
        if let Some(Ok(token)) = lexer.next() {
            match token {
                Token::Number(value) => Ok(FieldVal::Num(value)),
                Token::String(value) => Ok(FieldVal::Str(value)),
                _ => Err(ParseError {
                    ctx: ctx.to_owned(),
                    msg: "Expected field value (number or quoted string)".to_owned(),
                }),
            }
        } else {
            Err(ParseError {
                ctx: ctx.to_owned(),
                msg: "Found unexpected... something".to_owned(),
            })
        }
    }?;

    match lexer.next() {
        Some(Ok(Token::NewLine)) => Ok(Field { id, value }),
        _ => Err(ParseError {
            ctx: ctx.to_owned(),
            msg: "Expected new line".to_owned(),
        }),
    }
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
                value: FieldVal::Str("sample text"),
            },
            Field {
                id: 3,
                value: FieldVal::Num(-66),
            },
            Field {
                id: 4,
                value: FieldVal::Str("sth"),
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

    #[test]
    fn test_reject_trailing() {
        let input = "\
Record: 12
P01: 321
%\nP02";

        let obtained = parse_record(input);

        assert!(obtained.is_err())
    }
}
