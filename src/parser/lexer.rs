use logos::*;

fn parse_int(str: &str) -> i32 {
    str.parse::<i32>().expect("Valid number")
}

fn drop_quotes(str: &str) -> &str {
    let len = str.len();
    &str[1..len - 1]
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip " ")]
pub enum Token<'a> {
    #[token["Record:"]]
    RecordStart,
    #[token["%"]]
    RecordEnd,
    #[regex[r"P\d+:", |lex| parse_int(&lex.slice()[1..lex.slice().len() - 1])]]
    FieldId(i32),
    #[regex[r"-?\d+", |lex| parse_int(lex.slice())]]
    Number(i32),
    #[regex[r#""[^"]+""#, |lex| drop_quotes(lex.slice())]]
    String(&'a str),
    #[regex["\n"]]
    NewLine,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_record() {
        let input = "\
            Record: 12
            P01: 321
            P02: \"sample text\"
            P03:  -66
            P04:  \"sth\"
            %\n";

        let lexer = Token::lexer(input);

        let obtained: Vec<_> = lexer.collect();

        use Token::*;
        let expected = vec![
            Ok(RecordStart),
            Ok(Number(12)),
            Ok(NewLine),
            Ok(FieldId(1)),
            Ok(Number(321)),
            Ok(NewLine),
            Ok(FieldId(2)),
            Ok(String("sample text")),
            Ok(NewLine),
            Ok(FieldId(3)),
            Ok(Number(-66)),
            Ok(NewLine),
            Ok(FieldId(4)),
            Ok(String("sth")),
            Ok(NewLine),
            Ok(RecordEnd),
            Ok(NewLine),
        ];

        assert_eq!(obtained, expected)
    }
}
