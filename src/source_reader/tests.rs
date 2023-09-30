use std::io::BufReader;

use super::*;

const EXAMPLE_INPUT: &str = "\
Something to filter out at the beginning
Record: 1
P02: \"another sample text\"
P01: -321
%
Record: 2
P01: 321
P02: \"sample text\"
P03:  \"sth\"
%
Record: 3
P02: \"another sample text\"
g4rb4ge
P01: -123
%
Something to filter out at the end
";

const REC1: &str = "\
Record: 1
P02: \"another sample text\"
P01: -321
%
";

const REC2: &str = "\
Record: 2
P01: 321
P02: \"sample text\"
P03:  \"sth\"
%
";

const REC3: &str = "\
Record: 3
P02: \"another sample text\"
g4rb4ge
P01: -123
%
";

#[test]
fn test_read_record() {
    let br = BufReader::new(EXAMPLE_INPUT.as_bytes());
    let mut src_reader = SourceReader::new(br);

    let results: Vec<_> = (0..4)
        .map(|_| {
            src_reader
                .read_record()
                .expect("Error when reading a record")
        })
        .collect();

    assert_eq!(results[0], Some(REC1.to_owned()));
    assert_eq!(results[1], Some(REC2.to_owned()));
    assert_eq!(results[2], Some(REC3.to_owned()));
    assert_eq!(results[3], None);
}
