# RadiantKit App Template

A fresh [RadiantKit](https://github.com/radiant-labs/radiantkit) app, ready for hacking 🚀

## Getting Started

Install `rust` and `wasm-bindgen`.

### Build
```
cd runtime
wasm-pack build --target=web --release --scope radiantkit
```

### Run
```
cd ../app
npm i
npm run start
```

## Template Structure

### Runtime

Core Rust based runtime that specifes the nodes, messages and interactions.

### App

A React app that interacts with the runtime.
