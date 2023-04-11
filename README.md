# insertfmt

fast & easy CLI specialized to format MySQL INSERT queries.

format queries so that they look like a table.

![preview](./images/preview.gif)

## Installation

You can install the module directly with `cargo`.

```
cargo install insertfmt
```

You can also download the binary from releases.

The binary is self-sufficient with no dependencies, and can be put anywhere on
your PATH and run with `insertfmt` command.

## Basic Usage

To run the tool, run the command with a path argument:

```bash
insertfmt x.sql y.sql <...>
```

You can specify as many paths as you want.
