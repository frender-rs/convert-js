use convert_js::{jsv, ToJs};
use js_test::deep_equal;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn struct_with_types() {
    #[derive(ToJs)]
    struct MyField<V> {
        key: String,
        value: V,
    }
    {
        let v = MyField {
            key: "k".to_owned(),
            value: false,
        };
        assert!(deep_equal(
            &v.to_js(),
            jsv!({ key: "k", value: false }).as_ref()
        ));
    }
    {
        let v = MyField {
            key: "k".to_owned(),
            value: JsValue::NULL,
        };
        assert!(deep_equal(
            &v.to_js(),
            jsv!({ key: "k", value: jsv!(null) }).as_ref()
        ));
    }
}

#[wasm_bindgen_test]
fn struct_with_lifetimes() {
    #[derive(ToJs)]
    struct MyField<'a, K: ToString, V> {
        key: &'a K,
        value: &'a V,
    }

    {
        let value = js_sys::Array::new();
        let v = MyField {
            key: &"k",
            value: &value,
        };
        assert!(deep_equal(
            &v.to_js(),
            jsv!({ key: "k", value: jsv!([]) }).as_ref()
        ));
    }
    {
        let key = "k".to_owned();
        let v = MyField {
            key: &key,
            value: &key,
        };
        assert!(deep_equal(
            &v.to_js(),
            jsv!({ key: "k", value: "k" }).as_ref()
        ));
    }
}
