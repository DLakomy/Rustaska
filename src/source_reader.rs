use std::io::{BufRead, Error};

#[cfg(test)]
mod tests;

pub struct SourceReader<T: BufRead> {
    src: T,
}

impl<T: BufRead> SourceReader<T> {
    pub fn new(src: T) -> Self {
        SourceReader { src }
    }

    /// None means end of input
    pub fn read_record(&mut self) -> Result<Option<String>, Error> {
        let mut acc = String::new();
        let mut line = String::new();
        while 0 < self.src.read_line(&mut line)? {
            if line.starts_with("Record") {
                // record start
                acc.push_str(&line);
                // acc.push('\n');
            } else if line == "%\n" {
                // record end
                acc.push_str(&line);
                return Ok(Some(acc));
            // nonempty means we are in the middle of a record, not looking at garbage
            } else if !acc.is_empty() {
                acc.push_str(&line);
            }
            line.clear();
        }
        Ok(None)
    }
}
