use super::model::*;
use std::io::{self, Write};

use crate::parser::ParseError;

pub struct CsvWriter<T: Write> {
    numbers_sink: T,
    strings_sink: T,
    errors_sink: T,
}

const HEADER: &str = "rec,field,val\n";

impl<T: Write> CsvWriter<T> {
    pub fn new(
        mut numbers_sink: T,
        mut strings_sink: T,
        errors_sink: T,
    ) -> Result<Self, io::Error> {
        numbers_sink.write_all(HEADER.to_owned().as_bytes())?;
        strings_sink.write_all(HEADER.to_owned().as_bytes())?;

        Ok(CsvWriter {
            numbers_sink,
            strings_sink,
            errors_sink,
        })
    }
    pub fn write_result(&mut self, result: Result<Record, ParseError>) -> io::Result<()> {
        // fn write_field() -> io::Result<()> {}

        match result {
            Ok(rec) => {
                let rec_id = rec.id;
                rec.fields.into_iter().try_for_each(|field| {
                    let field_id = field.id;
                    match field.value {
                        FieldVal::Num(v) => self
                            .numbers_sink
                            .write_all(format!("{rec_id};{field_id};{v}\n").as_bytes()),
                        FieldVal::Str(v) => self
                            .strings_sink
                            .write_all(format!("{rec_id};{field_id};\"{v}\"\n").as_bytes()),
                    }
                })
            }
            Err(e) => self.errors_sink.write_all(e.to_string().as_bytes()),
        }
    }

    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        self.numbers_sink.flush()?;
        self.strings_sink.flush()?;
        self.errors_sink.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufWriter;

    use crate::model::{Field, FieldVal};

    use super::*;

    const MISSING_ELEM: &str = "An element should be here";

    struct Results {
        str: String,
        num: String,
        err: String,
    }

    fn run_for_result(result: Result<Record, ParseError>) -> Result<Results, io::Error> {
        let mut buffers: Vec<Vec<u8>> = (0..3).map(|_| Vec::new()).collect();
        {
            let writers: Vec<_> = buffers.iter_mut().map(BufWriter::new).collect();

            let mut csv_writer = {
                let mut iter = writers.into_iter();
                CsvWriter::new(
                    iter.next().expect(MISSING_ELEM),
                    iter.next().expect(MISSING_ELEM),
                    iter.next().expect(MISSING_ELEM),
                )
            }?;

            csv_writer.write_result(result)?;

            csv_writer.flush()?;
        }

        let results = {
            let mut iter = buffers
                .into_iter()
                .map(|b| String::from_utf8(b).expect("Should convert to string"));

            Results {
                num: iter.next().expect(MISSING_ELEM),
                str: iter.next().expect(MISSING_ELEM),
                err: iter.next().expect(MISSING_ELEM),
            }
        };

        Ok(results)
    }

    #[test]
    fn test_writing_rec() {
        let id = 66;
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
        let rec = Record { id, fields };

        let results = run_for_result(Ok(rec)).expect("Should succeed");

        let expected_num = HEADER.to_owned() + "66;1;321\n";
        let expected_str = HEADER.to_owned() + "66;2;\"sample text\"\n66;3;\"sth\"\n";

        assert_eq!(results.num, expected_num);
        assert_eq!(results.str, expected_str);
        assert_eq!(results.err, "");
    }

    #[test]
    fn test_writing_err() {
        let err = ParseError::new("Record 1".to_owned(), "Something is wrong".to_owned());
        let err_str = err.to_string();
        let results = run_for_result(Err(err)).expect("Should succeed");

        assert_eq!(results.num, HEADER);
        assert_eq!(results.str, HEADER);
        assert_eq!(results.err, err_str);
    }
}
