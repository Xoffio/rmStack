
Here is the code for the answer of the question [How to import a WASM module in WASM (Rust) and pass a String parameter](https://stackoverflow.com/questions/72618553/how-to-import-a-wasm-module-in-wasm-rust-and-pass-a-string-parameter/72705670?noredirect=1#comment128447848_72705670)

This folder contains two folders:
- `add`: directory containing the code that generates the wasm file to be imported in `wasm-test01`. (known as `imported_lib.rs` in the question)

To build the wasm file, go the the `add` folder 
```bash
cd add
```

and build it with
```bash
wasm-pack build
```
Then copy the file `add/pkg/add_bg.wasm` and paste it into the folder `wasm-test01/src/`

- `wasm-test01`: the main code. (known as `main_lib.rs` in the question)

To compile it into `wasm-test01`, and run:
```bash
wasm-pack build
```

To generate the `www` folder, run:
```bash
npm init wasm-app www
```

To run the tmp server go into the folder `wasm-test01/www` and run:
```bash
cd wasm-test01/www
npm run start
```

---

## Reference links:
- https://rustwasm.github.io/docs/book/game-of-life/hello-world.html
- https://rustwasm.github.io/wasm-bindgen/examples/import-js.html
- https://rustwasm.github.io/wasm-bindgen/reference/js-snippets.html
- https://rustwasm.github.io/wasm-bindgen/contributing/design/exporting-rust.html
- https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-js-imports/indexing-getter-setter-deleter.html
- https://docs.rs/js-sys/latest/js_sys/index.html
- https://docs.rs/wasm-bindgen/0.2.81/wasm_bindgen/index.html