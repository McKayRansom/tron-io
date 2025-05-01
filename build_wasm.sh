#!/bin/bash
set -e

rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --bin tron-io --release

rm -rf deploy/
mkdir deploy

cp static/* deploy/
cp -r assets/ deploy/assets
cp target/wasm32-unknown-unknown/release/tron-io.wasm deploy

