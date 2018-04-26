<p align="center">
  <h1 align="center">prettycli</h1>
  <h4 align="center">Print pretty terminal messages</h4>
  <br>
</p>

## Description:
A simple Rust crate with **no dependencies** to print colorful messages with emojis on the terminal.

## Usage:

```rust
extern crate prettycli;

use prettycli::*;

fn main() {
    error("Hello, world!");
    info("Hello, world!");
    warn("Hello, world!");
    wait("Hello, world!");
    critical("Hello, world!");
    command("Hello, world!");
    link("Hello, world!");
    misc("Hello, world!");
}
```

will output

<img src="https://user-images.githubusercontent.com/2767425/39296879-b532e770-495f-11e8-9821-fb0464126ca7.png" />

## Issue/Feedback:

log them in the [github issues](https://github.com/cg-cnu/rust-prettycli/issues) or hit me on [twitter](https://twitter.com/cgcnu).

## Inspiration:

* [prettycli-node](https://github.com/siddharthkp/prettycli)

## Like it?:

Please ⭐ this repo