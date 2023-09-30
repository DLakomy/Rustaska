use std::error::Error;
use std::fs::File;
use std::io::BufReader;
mod source_reader;

use crate::source_reader::SourceReader;

pub fn program() -> Result<(), Box<dyn Error>> {
    let f = File::open("example-files/ex1.lst")?;
    let br = BufReader::new(f);

    let src_reader = SourceReader::new(br);
    let record = src_reader.read_record()?;
    print!("{}", record.unwrap());
    Ok(())
}
