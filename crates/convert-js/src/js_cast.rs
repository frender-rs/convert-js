use shrinkwraprs::Shrinkwrap;
use wasm_bindgen::{JsCast, JsValue};

use crate::ToJs;

#[derive(Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct WrapJsCast<T: JsCast>(pub T);

impl<T: JsCast> ToJs for WrapJsCast<T> {
    fn to_js(&self) -> wasm_bindgen::JsValue {
        JsValue::from(&self.0)
    }
}
