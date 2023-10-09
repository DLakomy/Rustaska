use std::{error::Error, fmt::Display};

use super::model::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, line_ending, space0},
    combinator::{all_consuming, cut, map, map_res, opt, recognize},
    error::{context, convert_error, VerboseError},
    multi::many1,
    sequence::{delimited, pair, preceded, terminated},
    Finish, IResult, Parser,
};

// switching error type to VerboseError
type Res<T, U> = IResult<T, U, VerboseError<T>>;

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
    match parse_record_internal(input).finish() {
        Ok((_, rec)) => Ok(rec),
        Err(e) => Err(ParseError {
            ctx: input.chars().take_while(|&c| c != '\n').collect(),
            msg: convert_error(input, e),
        }),
    }
}

fn parse_rec_header(i: &str) -> Res<&str, Id> {
    let str = delimited(tag("Record: "), digit1, line_ending);
    map_res(context("Record header", str), str::parse)(i)
}

fn parse_num(i: &str) -> Res<&str, i32> {
    let r = recognize(opt(char('-')).and(digit1));
    map_res(context("number", r), str::parse).parse(i)
}
fn parse_field(i: &str) -> Res<&str, Field> {
    let p_field_number = delimited(char('P'), parse_num, char(':').and(space0));
    let p_string_field = map(
        context(
            "string field",
            preceded(
                char('\"'),
                cut(terminated(take_while1(|c| c != '"'), char('\"'))),
            ),
        ),
        |s: &str| FieldVal::Str(s.to_owned()),
    );
    let p_num_field = map(parse_num, FieldVal::Num);
    let p_field_value = alt((p_string_field, p_num_field));

    map(
        terminated(pair(p_field_number, p_field_value), line_ending),
        |(id, value)| Field { id, value },
    )(i)
}

fn parse_record_internal(i: &str) -> Res<&str, Record> {
    let fields = many1(parse_field);
    let rec = map(pair(parse_rec_header, fields), |(id, fields)| Record {
        id,
        fields,
    });
    all_consuming(terminated(rec, tag("%\n")))(i)
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
    fn test_parse_rec_header() {
        let result = parse_rec_header("Record: 66\n");
        assert_eq!(result, Ok(("", 66)))
    }

    #[test]
    fn test_parse_pos_num_field() {
        let result = parse_field("P01: 321\n");
        assert_eq!(
            result,
            Ok((
                "",
                Field {
                    id: 1,
                    value: FieldVal::Num(321)
                }
            ))
        )
    }

    #[test]
    fn test_parse_neg_num_field() {
        let result = parse_field("P01: -321\n");
        assert_eq!(
            result,
            Ok((
                "",
                Field {
                    id: 1,
                    value: FieldVal::Num(-321)
                }
            ))
        )
    }
    #[test]
    fn test_parse_str_field() {
        let result = parse_field("P01: \"aqq\"\n");
        assert_eq!(
            result,
            Ok((
                "",
                Field {
                    id: 1,
                    value: FieldVal::Str("aqq".to_owned())
                }
            ))
        )
    }

    #[test]
    fn test_parse_unclosed_str_field() {
        let result = parse_field("P01: \"aqq\\n");
        assert!(result.is_err())
    }

    #[test]
    fn test_parse_record() -> Result<(), ParseError> {
        let input = "\
Record: 12
P01: 321
P02: \"sample text\"
P03:  \"sth\"
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
