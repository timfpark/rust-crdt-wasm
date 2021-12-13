## rust-crdts wasm

Proof of concept to utilizing the `crdts` [rust crate](https://github.com/rust-crdt/rust-crdt) in Javascript via WebAssembly.

The chief challenge in utilizing `crdts` via WebAssembly is that `wasm-bindgen` can't currently handle Rust structures with type parameters (which `crdts` extensively utilizes).

As such, this proof of concept takes a wrapping approach, building materialized structs for a CRDT / type pair that they can be consumed in JavaScript.

See `src/lib.rs` for details on how the mechanics of this wrapping work and at `index.html` for what the JavaScript client looks like.

### Running POC Demo

1. Install the WASM toolchain for Rust if you haven't already:

```
$ rustup target add wasm32-unknown-unknown
```

2. Install `wasm-pack` toolchain:

```
$ cargo install wasm-pack
```

3. Build the demo in WebAssembly using `wasm-pack`:

```
$ wasm-pack build --target web
```

4. Start a static webserver in the root of the application:

```
$ python3 -m http.server
```

5. Launch your web browser and navigate to `http://localhost:8000` - open your console to see that indeed that the counter is successfully run in JavaScript and outputs a `1`.
