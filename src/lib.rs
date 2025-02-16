//! A `Read` that limits the number of bytes read from an underlying `Read`.
//!
//! ```
//! use limit_reader::LimitReader;
//! use std::{io::Read, fs::File};
//! let file = File::open("testdata/hello.txt").unwrap();
//! let mut reader = LimitReader::new(file, 5);
//! let mut buffer = Vec::new();
//! reader.read_to_end(&mut buffer).unwrap();
//! assert!(buffer.len() == 5);
//! assert_eq!(buffer, b"Hello");
//!```

use std::io::{Read, Result};

pub struct LimitReader<T: Read> {
    reader: T,
    limit: usize,
}

impl<T: Read> LimitReader<T> {
    pub fn new(reader: T, limit: usize) -> Self {
        LimitReader { reader, limit }
    }
}

impl<T: Read> Read for LimitReader<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let limit = self.limit.min(buf.len());
        let count = self.reader.read(&mut buf[..limit])?;
        self.limit -= count;
        Ok(count)
    }
}
