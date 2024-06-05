# Limit Reader

A simple reader that limits the number of bytes read from an underlying reader.

## Install

```shell
cargo add limit-reader
```

## Usage

```rust
fn foo() {
    let hello_txt = fs::File::open("file").unwrap();
    let mut reader = LimiterReader::new(hello_txt, 1024 * 1024);
    let mut r1 = vec![0u8];
    reader.read_to_end(&mut r1).unwrap();
}
```
