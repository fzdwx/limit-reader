use std::io;
use std::io::Read;

struct LimiterReader<T: Read> {
    reader: T,
    limit: usize,
    reader_count: usize,
}

impl<T: Read> LimiterReader<T> {
    fn new(reader: T, limit: usize) -> Self {
        LimiterReader {
            reader,
            limit,
            reader_count: 0,
        }
    }
}

impl<T: Read> Read for LimiterReader<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let max_read = self.limit.min(buf.len()); // min of limit and buf.len()
        let bytes_read = self.reader.read(&mut buf[..max_read])?;
        self.reader_count += 1;
        Ok(bytes_read)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Read;

    use crate::LimiterReader;

    #[test]
    fn test_read() {
        let hello_txt = fs::File::open("testdata/hello.txt").unwrap();
        let mut reader = LimiterReader::new(hello_txt, 5);
        let mut r1 = vec![0u8];
        reader.read_to_end(&mut r1).unwrap();

        let mut hello_txt = fs::File::open("testdata/hello.txt").unwrap();
        let mut r2 = vec![0u8];
        hello_txt.read_to_end(&mut r2).unwrap();
        assert_eq!(r1, r2);
    }
}
