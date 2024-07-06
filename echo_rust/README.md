# echo_rust

Rust implementation of venerable `echo` Unix command.

## Description

Implemention from Chapter 2 of [_Command-Line Rust_](https://learning.oreilly.com/library/view/command-line-rust/9781098109424/)
(O'Reilly, 2022/2024, ISBN 9781098109417).

## Usage

```
  $ cargo run -- --help
  $ cargo run -- foo bar baz
  $ cargo run -- -n foo bar baz
```

## Tests

Run all tests:
```
  $ cargo test
```

## See Also

- [https://github.com/kyclark/command-line-rust](https://github.com/kyclark/command-line-rust)
