
## Setup template

Copy template for a particular day (for example day 1), and download puzzle file:
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

Tries to download the puzzle input if it does not exist yet in the `puzzles` folder.
Make sure to copy the AoC session cookie to `.env`.

```shell
cargo run --release -- 1
```
