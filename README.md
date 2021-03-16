# tetris.rs

![build status](https://github.com/xvm32/tetris.rs/workflows/build/badge.svg) [![codecov](https://codecov.io/gh/xvm32/tetris.rs/branch/master/graph/badge.svg?token=rpZcLzfCIT)](https://codecov.io/gh/xvm32/tetris.rs)



This is the classic tetris game I wrote to have a bit of fun with Rust.

## Installation and playing

![screenshot](tetris.png)

```
cargo install --git https://github.com/xvm32/tetris.rs.git
```

Then run:

```
tetris-rs
```

## Development

You will need curses libraries:

```
sudo apt install libncurses5 libncurses5-dev
```

Fork the repository, and make sure to run `clippy` and `rustfmt` before submitting a PR. There are no tests at this moment as this is mostly a toy project.

## Licensing

Licensed under the [MIT License](https://opensource.org/licenses/MIT). For details, see [LICENSE](https://github.com/xvm32/tetris.rs/blob/master/LICENSE).

