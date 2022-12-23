# mousetrap-rs

Rust implementation of https://github.com/inconshreveable/mousetrap

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

## License

MIT
