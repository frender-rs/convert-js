use shrinkwraprs::Shrinkwrap;
use wasm_bindgen::{JsCast, JsValue};

use crate::{FromJs, ToJs};

#[derive(Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct WrapJsCast<T: JsCast>(pub T);

impl<T: JsCast> ToJs for WrapJsCast<T> {
    fn to_js(&self) -> wasm_bindgen::JsValue {
        JsValue::from(&self.0)
    }
}

impl<T: JsCast> FromJs for WrapJsCast<T> {
    type Error = JsValue;

    fn from_js(js_value: JsValue) -> Result<Self, Self::Error> {
        Ok(WrapJsCast(js_value.dyn_into()?))
    }
}
