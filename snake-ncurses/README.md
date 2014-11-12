snake-ncurses
=============

A terminal-based snake game implementation in Rust using ncurses

## Build & Run

```sh
$ cargo run
```

## Dependencies

The recommended way is to use the [nighly rust docker images](https://github.com/schickling/dockerfiles/tree/master/rust)

#### Docker

```sh
$ docker run --rm -it -v $(pwd):/source schickling/rust
```

#### Ubuntu

```sh
$ sudo apt-get install libncurses5-dev
```

## Screenshot

![](https://raw.githubusercontent.com/schickling/rust-examples/master/snake-ncurses/screenshot.png)

## License

MIT
