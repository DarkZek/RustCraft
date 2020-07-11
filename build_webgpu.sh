#!/bin/bash

if RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --target wasm32-unknown-unknown; then
    if wasm-bindgen --out-dir target/generated --web target/wasm32-unknown-unknown/debug/rustcraft.wasm; then

        cd ./target/generated/
        php -S localhost:9000 &
        ~/Downloads/firefox/firefox localhost:9000
    fi
fi
echo Good job you hobo
exit
