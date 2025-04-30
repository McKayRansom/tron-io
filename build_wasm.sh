#!/bin/bash
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown --bin tron-io

rm -r deploy/
mkdir deploy

cp static/* deploy/
cp -r assets/ deploy/assets
cp target/wasm32-unknown-unknown/release/tron-io.wasm deploy

