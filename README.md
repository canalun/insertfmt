[![Test status](https://github.com/canalun/insertfmt/actions/workflows/cargo_test.yml/badge.svg)](https://github.com/canalun/insertfmt/actions/workflows/cargo_test.yml)
[![Docs](https://docs.rs/insertfmt/badge.svg)](https://docs.rs/insertfmt)
[![Dependency status](https://deps.rs/repo/github/canalun/insertfmt/status.svg)](https://deps.rs/repo/github/canalun/insertfmt)

![Rust version](https://img.shields.io/badge/rust--version-1.68+-brightgreen.svg)
[![Crates.io](https://img.shields.io/crates/v/insertfmt.svg)](https://crates.io/crates/insertfmt)

# insertfmt

fast & easy CLI specialized to format MySQL INSERT queries.

format queries so that they look like a table.

![preview](https://raw.githubusercontent.com/canalun/insertfmt/main/images/preview.gif)

## Installation

You can download the binary from releases.

The binary is self-sufficient with no dependencies, and can be put anywhere on
your PATH and run with `insertfmt` command!!

Or, you can also install the module directly with `cargo`.

```
cargo install insertfmt
```

## Basic Usage

To run the tool, run the command with a path argument:

```bash
insertfmt x.sql y.sql <...>
```

You can specify as many paths as you want.

## Release Note

### v1.0.3

- fix bug of backslash disappearance!

### v1.0.1-1.0.2

- fix error msg

### v1.0.0

- HBD🎂
- vSQLfmt gets able to format 'INSERT' queries!
