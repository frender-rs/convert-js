# convert-js

[![Crates.io](https://img.shields.io/crates/v/convert-js?style=for-the-badge)](https://crates.io/crates/convert-js)
[![docs.rs](https://img.shields.io/docsrs/convert-js/latest?style=for-the-badge)](https://docs.rs/convert-js)
[![GitHub license](https://img.shields.io/github/license/frender-rs/convert-js?style=for-the-badge)](https://github.com/frender-rs/convert-js/blob/main/LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/frender-rs/convert-js?style=for-the-badge)](https://github.com/frender-rs/convert-js/stargazers)

[wip] convert between rust wasm and js

# Current State

This crate is **NOT production ready**. Development is in [alpha branch](https://github.com/frender-rs/convert-js/tree/alpha#readme),
with auto releasing.
[![Release](https://github.com/frender-rs/convert-js/actions/workflows/release.yml/badge.svg?branch=alpha)](https://github.com/frender-rs/convert-js/actions/workflows/release.yml)

You can find the latest alpha version in [crates.io](https://crates.io/crates/convert-js/versions).

```toml
convert-js = "1.0.0-alpha.11"
```

`ToJs` trait is available for common types.

```rust
use convert_js::ToJs;

let v = Some("my_str");
v.to_js(); // "my_str" in js

let v: Option<i32> = None;
v.to_js(); // undefined in js
```

`ToJs` derive macro is available for structs;

```rust
use convert_js::ToJs;

/// NewType style struct can contain any type impl ToJs
#[derive(ToJs)]
struct Wrap(String);

/// Tuple struct
#[derive(ToJs)]
struct KeyValuePair(String, Option<js_sys::Object>);

/// Object struct
#[derive(ToJs)]
struct Position {
    x: f64,
    y: f64,
    data: KeyValuePair,
}
```

# TODO

- [ ] derive `ToJs` for enum
- [ ] `flatten`
- [ ] `into`, `from` `try_from`
- [ ] `FromJs` trait and macro

# Why not `#[wasm_bindgen]`

[`wasm-bindgen`](https://rustwasm.github.io/docs/wasm-bindgen/reference/types/exported-rust-types.html) provides an api to export rust structs to js.

However, supported types are very limited. `convert-js` aims to support arbitrary data.

And it works more like a proxy between rust wasm and js runtime:
The data is actually in wasm memory and
`wasm-bindgen` generates a js class to proxy the data accessors and methods.
That is why we must call `free` manually on exported objects in js to let rust know when to drop the data.
(As `wasm-bindgen` documents, this issue may be resolved by [weak references proposal](https://rustwasm.github.io/docs/wasm-bindgen/reference/weak-references.html) in js).
`convert-js` is designed to _convert_ between rust and js rather than to proxy:

- for js types (`JsValue` and imported `#[wasm-bindgen] extern "C"` types, e.g. `js_sys::Array`, `web_sys::HtmlElement`), pass them directly with `wasm-bindgen`
- for rust types (primitives and arbitrary structs, enums), convert them to/from `JsValue`.

# Why not `serde` ?

[`serde`](https://serde.rs/) is a great framework
for deserializing and serializing rust data structures.
To communicate with js and wasm-bindgen,
both [`serde-json`](https://github.com/serde-rs/json) and [`serde-wasm-bindgen`](https://github.com/cloudflare/serde-wasm-bindgen) are helpful.
[`stdweb`](https://github.com/koute/stdweb) also provides the ability to pass values between rust wasm and rust.

However, the above libraries are all based on `serde`,
whose data model are very limited. For example, there is no way to represent `wasm_bindgen::closure::Closure` in serde data model.

Thus, I created `convert-js`, to provide serde like apis for js specific serialization and deserialization. This crate is greatly inspired by the crates mentioned before.
Thanks for the developers.
