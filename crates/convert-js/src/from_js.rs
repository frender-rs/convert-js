use std::convert::Infallible;

use wasm_bindgen::{JsCast, JsValue};

pub trait FromJs: Sized {
    type Error;
    fn from_js(js_value: JsValue) -> Result<Self, Self::Error>;
}

impl FromJs for JsValue {
    type Error = Infallible;

    fn from_js(js_value: JsValue) -> Result<Self, Self::Error> {
        Ok(js_value)
    }
}

impl FromJs for () {
    type Error = JsValue;

    fn from_js(js_value: JsValue) -> Result<Self, Self::Error> {
        if js_value.is_undefined() {
            Ok(())
        } else {
            Err(js_value)
        }
    }
}

impl<T: FromJs> FromJs for Option<T> {
    type Error = T::Error;

    fn from_js(js_value: JsValue) -> Result<Self, Self::Error> {
        if js_value.is_undefined() || js_value.is_null() {
            Ok(None)
        } else {
            T::from_js(js_value).map(Some)
        }
    }
}

macro_rules! impl_from_js {
    (
        try_into:
        $($t:ty)*
    ) => {$(
        impl FromJs for $t {
            type Error = <$t as TryFrom<JsValue>>::Error;

            fn from_js(js_value: JsValue) -> Result<Self, Self::Error> {
                js_value.try_into()
            }
        }
    )*};
    (
        dyn_into:
            $($t:ty)+
    ) => {$(
        impl FromJs for $t {
            type Error = JsValue;

            fn from_js(js_value: JsValue) -> Result<Self, Self::Error> {
                js_value.dyn_into()
            }
        }
    )*};
}

impl_from_js! {
    try_into:
    f64
}

impl_from_js! {
    dyn_into:
    js_sys::Intl::Collator
    js_sys::Intl::DateTimeFormat
    js_sys::Intl::NumberFormat
    js_sys::Intl::PluralRules
    js_sys::WebAssembly::CompileError
    js_sys::WebAssembly::Global
    js_sys::WebAssembly::Instance
    js_sys::WebAssembly::LinkError
    js_sys::WebAssembly::Memory
    js_sys::WebAssembly::Module
    js_sys::WebAssembly::RuntimeError
    js_sys::WebAssembly::Table
    js_sys::Array
    js_sys::ArrayBuffer
    // js_sys::ArrayIter
    js_sys::AsyncIterator
    js_sys::BigInt
    js_sys::BigInt64Array
    js_sys::BigUint64Array
    js_sys::Boolean
    js_sys::DataView
    js_sys::Date
    js_sys::Error
    js_sys::EvalError
    js_sys::Float32Array
    js_sys::Float64Array
    js_sys::Function
    js_sys::Generator
    js_sys::Int8Array
    js_sys::Int16Array
    js_sys::Int32Array
    // js_sys::IntoIter
    // js_sys::Iter
    js_sys::Iterator
    js_sys::IteratorNext
    js_sys::JsString
    js_sys::Map
    js_sys::Number
    js_sys::Object
    js_sys::Promise
    js_sys::Proxy
    js_sys::RangeError
    js_sys::ReferenceError
    js_sys::RegExp
    js_sys::Set
    js_sys::SharedArrayBuffer
    js_sys::Symbol
    js_sys::SyntaxError
    js_sys::TypeError
    js_sys::Uint8Array
    js_sys::Uint8ClampedArray
    js_sys::Uint16Array
    js_sys::Uint32Array
    js_sys::UriError
    js_sys::WeakMap
    js_sys::WeakSet
}
