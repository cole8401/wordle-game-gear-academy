name: CI

on:
  pull_request:
    branches: [master]
  push:
    branches: [master]

env:
  CARGO_TERM_COLOR: always

jobs:
  ci:
    name: Run checks and tests
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Set up Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      - name: Check and test
        run: |
          cargo fmt --all --check
          cargo clippy --all-targets -- -D warnings -A unused-imports
          cargo t