# Bevy meshtext sample

Demonstrates how to use [meshtext](https://github.com/FrankenApps/meshtext) with [bevy](https://github.com/bevyengine/bevy).

## How to use
#### Desktop
1. Run using cargo `cargo run --release`

#### WASM
1. Build for wasm using cargo `cargo build --release --target wasm32-unknown-unknown`
2. Create JS binding with wasm-bindgen `wasm-bindgen --out-dir docs --target web target/wasm32-unknown-unknown/release/meshtext_bevy_sample.wasm`
3. Serve the root directory with a http-server. For example [`live-server`](https://www.npmjs.com/package/live-server)