#!/usr/bin/env bash

cargo build --release --target wasm32-unknown-emscripten &&
cp target/wasm32-unknown-emscripten/debug/luaweb.js . && 
cp target/wasm32-unknown-emscripten/debug/luaweb.wasm .