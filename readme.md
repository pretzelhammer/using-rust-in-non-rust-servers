# Using Rust in non-Rust Servers to Improve Performance

Companion code repository for the article [Using Rust in non-Rust Servers to Improve Performance](#).



## Setup

Required tools:
- `nvm` (Node version manager, [how to install](https://github.com/nvm-sh/nvm#installing-and-updating))
- `rustup` (Rust toolchain manager, [how to install](https://rustup.rs/))
- `just` (Command runner, `cargo install just`)
- `wasm-pack` (Builds Rust Wasm, `cargo install wasm-pack`)
- `wasm-opt` (Optimizes Wasm, `apt install binaryen`)

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

Also, the `just` commands `monitor` and `attack-and-monitor` will only work on Linux, macOS, or WSL because it relies on reading the `proc` directory.



## Commands

Get a list of all commands:
```
just list
```

Commands to run servers:
```
just tier-0-server
just tier-1-server
just tier-2-server
just tier-3-server
just tier-4-server
```

Command to benchmark currently running server:
```
just attack-and-monitor
```

## License

This code is dual-licensed under [Apache License Version 2.0](./license-apache) or [MIT License](./license-mit), at your option.