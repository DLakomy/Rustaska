use std::{error::Error, fmt::Display};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, line_ending, space0},
    combinator::{all_consuming, map, map_res, opt, recognize},
    multi::many1,
    sequence::{delimited, terminated, tuple},
    IResult, Parser,
};

// could be usize, no idea;
// not important in this case, it's a toy project
type Id = i32;

#[derive(PartialEq, Eq, Debug)]
enum FieldVal {
    Num(i32),
    Str(String),
}
#[derive(PartialEq, Eq, Debug)]
struct Field {
    id: Id,
    value: FieldVal,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Record {
    id: Id,
    fields: Vec<Field>,
}

#[derive(Debug)]
pub struct ParseError {
    msg: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for ParseError {}
// Record: 1
// P02: "another sample text"
// P01: -321
// %

// TODO maybe String will do? ie. str instead of ParseError
pub fn parse_record(input: &str) -> Result<Record, ParseError> {
    todo!();
}

fn parse_rec_header(i: &str) -> IResult<&str, Id> {
    let str = delimited(tag("Record: "), digit1, line_ending);
    map_res(str, str::parse)(i)
}

fn parse_num(i: &str) -> IResult<&str, i32> {
    let r = recognize(opt(char('-')).and(digit1));
    map_res(r, str::parse).parse(i)
}

fn parse_field(i: &str) -> IResult<&str, Field> {
    let p_field_number = delimited(char('P'), parse_num, char(':').and(space0));
    let p_string_field = map(
        delimited(char('"'), take_while1(|c| c != '"'), char('"')),
        |s: &str| FieldVal::Str(s.to_owned()),
    );
    let p_num_field = map(parse_num, FieldVal::Num);
    let p_field_value = alt((p_string_field, p_num_field));

    tuple((p_field_number, p_field_value, line_ending))
        .map(|(k, v, _)| Field { id: k, value: v })
        .parse(i)
}

fn parse_record_internal(i: &str) -> IResult<&str, Record> {
    let fields = many1(parse_field);
    let mut rec = map(tuple((parse_rec_header, fields)), |(id, fields)| Record {
        id,
        fields,
    });
    all_consuming(terminated(rec, tag("%\n")))(i)
}

// TODO VerboseError, preceded(char('\"'), cut(terminated(parse_str, char('\"')))) - for string parsing in str field

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_parse_record() {
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

        let result = parse_record_internal(input);
        assert_eq!(result, Ok(("", expected)))
    }
}
