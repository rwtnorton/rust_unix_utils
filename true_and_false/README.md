# true_and_false

Rust implementation of venerable `true` and `false` Unix commands.

## Description

Implemention from Chapter 1 of [_Command-Line Rust_](https://learning.oreilly.com/library/view/command-line-rust/9781098109424/)
(O'Reilly, 2022/2024, ISBN 9781098109417).

## Usage

```
  $ cargo run --quiet --bin true; echo $?
  $ cargo run --quiet --bin false; echo $?
```

## Tests

Run all tests:
```
  $ cargo test
```

Run test for `true` executable:
```
  $ cargo test --quiet --bin true
```

Run test for `false` executable:
```
  $ cargo test --quiet --bin false
```


## See Also

- [https://github.com/kyclark/command-line-rust](https://github.com/kyclark/command-line-rust)
