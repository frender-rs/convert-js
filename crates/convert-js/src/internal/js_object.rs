use wasm_bindgen::JsValue;

use crate::ToJs;

pub struct JsObject(js_sys::Object);

impl JsObject {
    #[inline]
    pub fn new() -> Self {
        Self(js_sys::Object::new())
    }

    #[inline]
    pub fn into_inner(self) -> js_sys::Object {
        self.0
    }

    #[inline]
    pub fn into_js_value(self) -> JsValue {
        self.0.into()
    }

    #[inline]
    pub fn with_prop<K: ToJs, V: ToJs>(self, key: &K, v: &V) -> Self {
        self.set_prop(key, v);
        self
    }

    #[inline]
    pub fn set_prop<K: ToJs, V: ToJs>(&self, key: &K, v: &V) -> &Self {
        if let Some(v) = v.to_js_property_value() {
            let k = key.to_js();
            js_sys::Reflect::set(&self.0, &k, &v).unwrap();
        }

        self
    }
}
