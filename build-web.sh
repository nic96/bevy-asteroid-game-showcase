cargo build --target wasm32-unknown-unknown --target-dir target
wasm-bindgen --out-dir public/target --target web target/wasm32-unknown-unknown/debug/bevy-asteroid-game-showcase.wasm
cp -r assets public/
python -m http.server -d public
