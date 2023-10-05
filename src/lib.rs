mod parser;
mod source_reader;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use crate::source_reader::SourceReader;

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    source_path: String,
    numbers_path: String,
    strings_path: String,
    errors_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // ignore first
        let source_path = args.next().ok_or("Didn't get a source path")?;
        let numbers_path = args.next().ok_or("Didn't get a numbers destination path")?;
        let strings_path = args.next().ok_or("Didn't get a strings destination path")?;
        let errors_path = args.next().ok_or("Didn't get an error file path")?;

        if args.next().is_some() {
            Err("Too many arguments")
        } else {
            Ok(Config {
                source_path,
                numbers_path,
                strings_path,
                errors_path,
            })
        }
    }
}

pub fn program(cfg: Config) -> Result<(), Box<dyn Error>> {
    let f = File::open("example-files/ex1.lst")
        .map_err(|e| format!("Error while opening source: {e}"))?;
    let br = BufReader::new(f);

    let mut src_reader = SourceReader::new(br);
    let record = src_reader.read_record()?;
    print!("{}", record.unwrap());
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_config_parser() {
        // happy path
        let args = vec![
            "0".to_owned(),
            "src".to_owned(),
            "num".to_owned(),
            "str".to_owned(),
            "err".to_owned(),
        ]
        .into_iter();

        let expected = Config {
            source_path: "src".to_owned(),
            numbers_path: "num".to_owned(),
            strings_path: "str".to_owned(),
            errors_path: "err".to_owned(),
        };
        let obtained = Config::build(args);

        assert_eq!(Ok(expected), obtained);

        // unhappy path 1 - too few args
        let args = vec![
            "0".to_owned(),
            "src".to_owned(),
            "num".to_owned(),
            "err".to_owned(),
        ]
        .into_iter();

        let obtained = Config::build(args);
        assert!(obtained.is_err());

        // unhappy path 2 - too many args
        let args = vec![
            "0".to_owned(),
            "src".to_owned(),
            "num".to_owned(),
            "str".to_owned(),
            "err".to_owned(),
            "sth".to_owned(),
        ]
        .into_iter();

        let obtained = Config::build(args);
        assert!(obtained.is_err());
    }
}
