# Wasm-sass

## 🏗 Development

### 🛠️ Build with `wasm-pack`

_Run build script_
```bash
sh ./scripts/build.sh
```

_or manually:_
```bash
wasm-pack build --target nodejs --scope jonshort
```

### 🔬 Test in node with `wasm-pack test`

_Run test script_
```bash
sh ./scripts/test.sh
```

_or manually:_
```bash
wasm-pack test --node
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
