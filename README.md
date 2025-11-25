
## Setup template and download puzzle data

```shell
cargo run --release -- 1 --download
```

Note, doing this out of order is not supported.

## Run tests

Tests are in `src/test`, one file for each day.

Run a test for a particular day (for example day 1):
```shell
cargo test test01 --release
```

## Run actual puzzle

```shell
cargo run --release -- 1
```
