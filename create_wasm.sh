cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --out-name bevy_game --out-dir wasm --target web target/wasm32-unknown-unknown/release/logic-turn-based-rpg.wasm
rm -r wasm/assets
cp -r assets wasm/
cd wasm
python3 -m http.server