# mousetrap-rs

[![Crates.io](https://img.shields.io/crates/v/mousetrap?style=flat-square)](https://crates.io/crates/mousetrap)
[![Docs.rs](https://img.shields.io/docsrs/mousetrap?style=flat-square)](https://docs.rs/mousetrap)

Rust implementation of https://github.com/inconshreveable/mousetrap, which is used in ngrok.

> mousetrap is a tiny library that answers a single question.
>
> On a Windows machine, was the process invoked by someone double-clicking on the executable file while browsing in explorer?

> `mousetrap` provides a way to detect these invocations so that you can provide more helpful behavior and instructions on how to run the CLI tool.

On non-Windows platforms, this library always returns `false`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mousetrap = "0.1"
```

## Usage

```rust
use std::io;

fn main() {
    println!("started_by_explorer: {}", mousetrap::started_by_explorer());

    println!("Press Enter to exit...");

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
}
```

## More

This is my first contribution to the Rust community. I hope you find it useful.

And I'm not familiar with Rust currently, so if you have any suggestions, feel free to open an issue or PR.

## License

MIT
