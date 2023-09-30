use std::io::{BufRead, Error};

pub struct SourceReader<T: BufRead> {
    src: T,
}

impl<T: BufRead> SourceReader<T> {
    pub fn new(src: T) -> Self {
        SourceReader { src }
    }

    /// None means end of input
    pub fn read_record(mut self) -> Result<Option<String>, Error> {
        // TODO actually read record by record + unit tests
        let mut bf = String::new();
        self.src.read_line(&mut bf)?;
        Ok(Some(bf))
    }
}
