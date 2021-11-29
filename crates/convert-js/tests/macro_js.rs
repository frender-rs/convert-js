use convert_js::jsv;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn undefined() {
    let v = jsv!(undefined);
    assert!(v.is_undefined());
}

#[wasm_bindgen_test]
fn null() {
    let v = jsv!(null);
    assert!(v.is_null());
}

#[wasm_bindgen_test]
fn literal() {
    {
        let v = jsv!(1);
        assert!(v.as_f64().is_some());
        assert_eq!(v, 1);
    }
    {
        let v = jsv!("abc");
        assert!(v.is_string());
        assert_eq!(v, "abc");
    }
}

#[wasm_bindgen_test]
fn object() {
    {
        let obj = jsv!({});
        assert!(js_test::object::is_empty_object_with_object_proto(&obj));
    }
    {
        let obj = jsv!({ "k": true });
        assert!(js_test::deep_equal(
            &obj,
            &js_sys::JSON::parse(r#"{ "k": true }"#).unwrap()
        ));
    }
    {
        let v = [1, 2];
        let obj = jsv!({
            "k": jsv!({
                1: 0.5,
                v,
                arr: jsv!([1, 2, "3"]),
            })
        });
        assert!(js_test::deep_equal(
            &obj,
            &js_sys::JSON::parse(r#"{ "k": { "1": 0.5, "v": [1, 2], "arr": [1, 2, "3"] } }"#)
                .unwrap()
        ));
    }
}
