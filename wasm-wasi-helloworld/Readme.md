cargo build --target wasm32-wasi --release

node --experimental-wasi-unstable-preview1 run.js
