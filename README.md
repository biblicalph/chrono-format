# Chrono Strftime Validator

This package provides a utility function for validating an [strftime](https://docs.rs/chrono/latest/chrono/format/strftime/index.html#specifiers) format.

The project is built in Rust and compiles to WebAssembly using [wasm-pack](https://github.com/rustwasm/wasm-pack)

## Usage

```js
const { is_valid_strftime_format } = require('chrono-format');

// returns true
is_valid_strftime_format('%FT%T') 
// returns false
is_valid_strftime_format('%QT%T') 
```

### Build

```
wasm-pack build --target nodejs
```

### Tests

```
wasm-pack test --node
```

### 🎁 Publish to NPM

```
wasm-pack publish
```


### Acknowledgements

<sub>Built with 🦀🕸 by [The Rust and WebAssembly Working Group](https://rustwasm.github.io/)</sub>