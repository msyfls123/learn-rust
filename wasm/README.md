# Serve


# Used to compile your Rust code to WebAssembly
```
cargo install wasm-pack
cd client
chmod +x ./build.sh
./build.sh
cd -
cargo run
```

### Visit localhost:7878/?init=77 in your browser, have fun

~~Or any other static file server that supports the application/wasm mime type (deprecated)~~

~~npm install -g http-server~~
