# wasm-console
![CI](https://github.com/nwtgck/wasm-console/workflows/CI/badge.svg)

`console.log()` in WebAssembly in the same way as JavaScript 

## Usage

```toml
# Cargo.toml

[dependencies]
wasm-console = { git = "https://github.com/nwtgck/wasm-console" }
```

Here is an usage. The most important thing is that you can specify multiple values as the same as `console.log` in JavaScript.

```rust
wasm_console::log!("Hi!", true, 1.3);
```

Any `x` which can be `JsValue::from(x)` can be specified the arguments.

## Example
Here is an example using wasm-console.
- <https://github.com/nwtgck/wasm-console-example>
