# Using Rust in Non-Rust Servers to Improve Performance

Companion code repository for the article [Using Rust in Non-Rust Servers to Improve Performance](https://github.com/pretzelhammer/rust-blog/blob/master/posts/rust-in-non-rust-servers.md).



## Setup

Required tools:
- `nvm` (Node version manager, [how to install](https://github.com/nvm-sh/nvm#installing-and-updating))
- `rustup` (Rust toolchain manager, [how to install](https://rustup.rs/))
- `just` (Command runner, `cargo install just`)
- `wasm-pack` (Builds Rust Wasm, `cargo install wasm-pack`)
- `wasm-opt` (Optimizes Wasm, `apt install binaryen` or `brew install binaryen`)

Then from the root of this project run:
```bash
# install specific version of node
# and download project dependencies
cd ./node && nvm install && npm ci

# update to latest stable Rust toolchain
rustup update stable
```

Optional tools, for running benchmarks:
- `vegeta` (HTTP load tester, [how to install](https://github.com/tsenart/vegeta#install))

Also, the `just` commands `monitor` and `attack-and-monitor` will only work on Linux or WSL because they rely on reading the `proc` directory to measure a process's CPU and RAM usage. However, you can still run `just attack` on any OS.



## Commands

Get a list of all commands:
```bash
just list
```

Commands to run servers:
```bash
just tier-0-server
just tier-1-server
just tier-2-server
just tier-3-server
just tier-4-server
```

Command to benchmark currently running server:
```bash
# on Linux or WSL
just attack-and-monitor

# on macOS or Windows
just attack
```



## License

This code is dual-licensed under [Apache License Version 2.0](./license-apache) or [MIT License](./license-mit), at your option.
