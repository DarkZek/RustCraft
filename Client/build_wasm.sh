cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./target/site/ --target web ./target/wasm32-unknown-unknown/release/bevy-testing.wasm