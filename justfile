#!/usr/bin/env just --justfile

release:
  cargo build --release

pub version:
    sed -i  -e "s/^version = .*/version = \"{{version}}\"/" Cargo.toml
    git tag -a {{version}} -m "{{version}}"
    cargo publish --registry crates-io
