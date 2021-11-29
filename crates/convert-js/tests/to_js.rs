use convert_js::{Maybe, Nullable, ToJs};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn tuple_as_js() {
    assert!(().to_js().is_undefined());

    {
        let js = (
            (),
            1,
            "2",
            JsValue::from_f64(3.0),
            Nullable::NonNull(4u128),
            Maybe::Defined(false),
        )
            .to_js();

        let arr: js_sys::Array = js.dyn_into().unwrap();

        assert_eq!(arr.length(), 6);
        assert!(arr.get(0).is_undefined());
        assert_eq!(arr.get(1), 1);
        assert_eq!(arr.get(2), "2");
        assert_eq!(arr.get(3), 3.0);
        assert!(arr.get(4).is_bigint());
        assert_eq!(arr.get(4), 4u128);
        assert_eq!(arr.get(5), false);
    }
}

#[wasm_bindgen_test]
fn array_as_js() {
    let js = [Some(1), None, Some(-1)].to_js();
    let arr: js_sys::Array = js.dyn_into().unwrap();

    assert_eq!(arr.length(), 3);
    assert_eq!(arr.get(0), 1);
    assert!(arr.get(1).is_undefined());
    assert_eq!(arr.get(2), -1);
}

#[wasm_bindgen_test]
fn vec_as_js() {
    use convert_js::Nullable;
    let js = vec![
        Nullable::Null,
        Nullable::NonNull(Box::new("1")),
        Nullable::NonNull(Box::new("2")),
    ]
    .to_js();
    let arr: js_sys::Array = js.dyn_into().unwrap();

    assert_eq!(arr.length(), 3);
    assert!(arr.get(0).is_null());
    assert_eq!(arr.get(1), "1");
    assert_eq!(arr.get(2), "2");
}

#[wasm_bindgen_test]
fn slice_as_js() {
    let slice = "abcde".as_bytes();

    let js = slice.to_js();
    let arr: js_sys::Array = js.dyn_into().unwrap();

    assert_eq!(arr.length(), 5);
    assert_eq!(arr.get(0), 97);
    assert_eq!(arr.get(1), 98);
    assert_eq!(arr.get(2), 99);
    assert_eq!(arr.get(3), 100);
    assert_eq!(arr.get(4), 101);
}
