# bevy-asteroid-game-showcase
Just a simple rocket/asteroid game written in Rust using Bevy


Note: WIP/this is just for me to test out Bevy stuff and learn.

## WASM
![WASM Demo](https://github.com/nic96/bevy-asteroid-game-showcase/blob/master/screenshots/rocket-bevy-showcase.gif "Preview Gif")

Check it out here: [jeromyreimer.com/bevy-showcase/rocket](https://jeromyreimer.com/bevy-showcase/rocket/)


This is how I built the wasm version that is currently in the wasm branch:
```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo build --target wasm32-unknown-unknown
wasm-bindgen --out-dir public/target --target web target/wasm32-unknown-unknown/debug/bevy_playground.wasm
python -m http.server -d public
```
