#!/bin/bash
set -e

rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --bin tron-io-client --release

rm -rf deploy/
mkdir deploy

cp static/* deploy/
cp -r client/assets/ deploy/assets
cp target/wasm32-unknown-unknown/release/tron-io-client.wasm deploy

