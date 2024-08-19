### Hyperliquid

[![Rust](https://github.com/dennohpeter/hyperliquid/actions/workflows/general.yml/badge.svg)](https://github.com/dennohpeter/hyperliquid/actions/workflows/general.yml)
[![Rust](https://github.com/dennohpeter/hyperliquid/actions/workflows/audit.yml/badge.svg)](https://github.com/dennohpeter/hyperliquid/actions/workflows/audit.yml)
[![](https://img.shields.io/badge/License-MIT-green.svg)](./LICENSE)
[![](https://img.shields.io/crates/v/hyperliquid)](https://crates.io/crates/hyperliquid)

### About

A Rust library for Hyperliquid API

### Install

`Cargo.toml`

```toml
[dependencies]

hyperliquid = { version = "0.2.4" }
```

### Usage

```rust
use hyperliquid::{Hyperliquid, Chain, Info};

#[tokio::main]
async fn main() {
    let user: Address = "0xc64cc00b46101bd40aa1c3121195e85c0b0918d8"
        .parse()
        .expect("Invalid address");


    let info:Info = Hyperliquid::new(Chain::Dev);

    // Retrieve exchange metadata
    let metadata = info.metadata().await.unwrap();
    println!("Metadata \n{:?}", metadata.universe);
}
```

### Examples

See `examples/` for examples. You can run any example with `cargo run --example <example_name>`.
