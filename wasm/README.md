
## Serve

```
# Used to compile your Rust code to WebAssembly
cargo install wasm-pack

# Or any other static file server that supports the application/wasm mime type
npm install -g http-server

chmod +x ./build.sh
./build.sh

# Visit localhost:8080 in your browser
http-server ./public --open
```