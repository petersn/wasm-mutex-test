#!/bin/bash
set -x
set -e

export RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals'
cargo build --lib --target wasm32-unknown-unknown --release -Zbuild-std=std,panic_abort
wasm-bindgen ./target/wasm32-unknown-unknown/release/wasm_mutex_test.wasm --out-dir ./web/pkg --target no-modules

