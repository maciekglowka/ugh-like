cargo build --target wasm32-unknown-unknown --release
rm ./wasm-out -rf
wasm-bindgen --out-dir ./wasm-out/ --target web ./target/wasm32-unknown-unknown/release/grrr.wasm
cp ./wasm-assets/index.html ./wasm-out
cd wasm-out && zip -r build.zip *