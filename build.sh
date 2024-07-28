#!/usr/bin/env bash

cd ./wasm || exit
wasm-pack build --target web
mv pkg/wasm_bg.wasm ../web/
mv pkg/wasm.js ../web
