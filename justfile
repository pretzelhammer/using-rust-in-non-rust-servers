### General commands ###

# list commands
default:
    @just --list

# list commands
list:
    @just --list

# delete qr-codes dir
clean-qr-codes:
    rm -rf ./qr-codes

# stress test the server
attack:
    cat targets.txt | vegeta attack -duration=30s -rate=0 -max-workers=100 | vegeta report

# monitor resource usage of server listening on 42069
monitor: build-monitor-process
    #!/usr/bin/env bash
    pid=$(lsof -t -i :42069)
    if [ -z "$pid" ]; then
        echo "No process is listening on port 42069"
        exit 1
    fi
    ./rust/target/release/monitor-process "$pid"

# attack and monitor in one command
attack-and-monitor: build-monitor-process
    #!/usr/bin/env bash
    pid=$(lsof -t -i :42069)
    if [ -z "$pid" ]; then
        echo "No process is listening on port 42069"
        exit 1
    fi
    ./rust/target/release/monitor-process "$pid" --quiet &
    monitor_pid=$!
    sleep 1
    just attack
    kill -SIGINT "$monitor_pid"
    sleep 1



### Node commands ###

# run single-threaded tier-0 node server
tier-0-single-threaded:
    node ./node/server.js tier-0 single-threaded

# run multi-threaded tier-0 node server
tier-0-multi-threaded:
    node ./node/server.js tier-0 multi-threaded

# run tier-0 node server
tier-0-server: tier-0-multi-threaded

# run single-threaded tier-1 node server
tier-1-single-threaded: build-qr-cli
    node ./node/server.js tier-1 single-threaded

# run multi-threaded tier-1 node server
tier-1-multi-threaded: build-qr-cli
    node ./node/server.js tier-1 multi-threaded

# run tier-1 node server
tier-1-server: tier-1-multi-threaded

# run single-threaded tier-2 node server
tier-2-single-threaded: build-qr-wasm
    node ./node/server.js tier-2 single-threaded

# run multi-threaded tier-2 node server
tier-2-multi-threaded: build-qr-wasm
    node ./node/server.js tier-2 multi-threaded

# run tier-2 node server
tier-2-server: tier-2-multi-threaded

# run single-threaded tier-2 node server
tier-2-bindgen-single-threaded: build-qr-wasm-bindgen
    node ./node/server.js tier-2-bindgen single-threaded

# run multi-threaded tier-2 node server
tier-2-bindgen-multi-threaded: build-qr-wasm-bindgen
    node ./node/server.js tier-2-bindgen multi-threaded

# run tier-2 node server
tier-2-bindgen-server: tier-2-bindgen-multi-threaded

# run single-threaded tier-3 node server
tier-3-single-threaded: build-qr-napi
    node ./node/server.js tier-3 single-threaded

# run multi-threaded tier-3 node server
tier-3-multi-threaded: build-qr-napi
    node ./node/server.js tier-3 multi-threaded

# run tier-3 node server
tier-3-server: tier-3-multi-threaded



### Rust commands ###

# run release qr-cli
qr-cli text: build-qr-cli
    mkdir -p ./qr-codes
    ./rust/target/release/qr-cli {{text}} > ./qr-codes/{{text}}.png
    echo generated ./qr-codes/{{text}}.png

# build release qr-cli
build-qr-cli:
    cd ./rust && RUSTFLAGS="-C target-cpu=native" cargo build --release -p qr-cli
    cp ./rust/target/release/qr-cli ./node/tier-1-rust-cli

# run debug qr-cli
debug-qr-cli text: build-debug-qr-cli
    mkdir -p ./qr-codes
    ./rust/target/debug/qr-cli {{text}} > ./qr-codes/{{text}}.png
    echo generated ./qr-codes/{{text}}.png

# build debug qr-cli
build-debug-qr-cli:
    cd ./rust && cargo build -p qr-cli

# build release qr-wasm
build-qr-wasm:
    cd ./rust && cargo build --release --target wasm32-unknown-unknown -p qr-wasm
    wasm-opt -O4 --all-features ./rust/target/wasm32-unknown-unknown/release/qr_wasm.wasm -o ./node/tier-2-rust-wasm/qr_wasm.wasm

# build debug qr-wasm
build-debug-qr-wasm:
    cd ./rust && cargo build --target wasm32-unknown-unknown -p qr-wasm

# build qr-wasm-bindgen
build-qr-wasm-bindgen:
    cd ./rust/qr-wasm-bindgen && wasm-pack build --release --target nodejs
    cp ./rust/qr-wasm-bindgen/pkg/qr_wasm_bindgen* ./node/tier-2-rust-wasm-bindgen/

# build debug qr-wasm-bindgen
build-debug-qr-wasm-bindgen:
    cd ./rust/qr-wasm-bindgen && wasm-pack build --debug --target nodejs

qr-napi-cdylib := if os() == "linux" {
    "libqr_napi.so"
} else if os() == "macos" {
    "libqr_napi.dylib"
} else if os() == "windows" {
    "qr_napi.dll"
} else {
    "unknown_os"
}

# build release qr-napi
build-qr-napi:
    cd ./rust && cargo build --release -p qr-napi
    cp ./rust/target/release/{{qr-napi-cdylib}} ./node/tier-3-rust-native/qr_napi.node

# build debug qr-napi
build-debug-qr-napi:
    cd ./rust && cargo build -p qr-napi

# run tier-4 rust server
tier-4-server:
    cd ./rust && RUSTFLAGS="-C target-cpu=native" cargo run --release -p qr-server

# run debug qr-server
debug-qr-server:
    cd ./rust && cargo run -p qr-server

# build prod qr-server
build-qr-server:
    cd ./rust && RUSTFLAGS="-C target-cpu=native" cargo build --release -p qr-server

# build debug qr-server
build-debug-qr-server:
    cd ./rust && cargo build -p qr-server

# build monitor-process
build-monitor-process:
    cd ./rust && cargo build --release -p monitor-process

# build debug monitor-process
build-debug-monitor-process:
    cd ./rust && cargo build -p monitor-process
