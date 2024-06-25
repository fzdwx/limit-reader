#!/usr/bin/env just --justfile

release:
  cargo build --release

pub version:
    sed -i  -e "s/^version = .*/version = \"{{version}}\"/" Cargo.toml
    cargo fmt
    cargo check
    git tag {{version}}
    git add .
    git commit -m "Bump version to {{version}}"
    cargo publish --registry crates-io
