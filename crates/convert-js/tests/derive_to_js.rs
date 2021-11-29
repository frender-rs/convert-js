use js_test::deep_equal;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_test::wasm_bindgen_test;

use convert_js::{jsv, Maybe, Nullable, ToJs, WrapJsCast};

#[wasm_bindgen_test]
fn empty_struct() {
    #[derive(ToJs)]
    struct EmptyObj {}

    let v = EmptyObj {};

    let obj = v.to_js();

    assert_eq!(js_sys::JSON::stringify(&obj).unwrap(), "{}");

    let obj: js_sys::Object = obj.dyn_into().unwrap();
    let keys = js_sys::Object::keys(&obj);

    assert_eq!(keys.length(), 0);
}

#[wasm_bindgen_test]
fn object_single_prop() {
    #[derive(ToJs)]
    struct MyObj {
        my_prop: i32,
    }

    let v = MyObj { my_prop: 1 };

    let js = v.to_js();

    assert_eq!(js_sys::JSON::stringify(&js).unwrap(), r#"{"my_prop":1}"#);

    let obj: js_sys::Object = js.dyn_into().unwrap();
    let keys = js_sys::Object::keys(&obj);

    assert_eq!(keys.length(), 1);
    assert_eq!(keys.get(0), "my_prop");

    let value = js_sys::Reflect::get(&obj, &"my_prop".into()).unwrap();

    assert!(value.as_f64().is_some());
    assert_eq!(value, 1);
}

#[wasm_bindgen_test]
fn object_optional_prop() {
    use convert_js::Nullable;

    #[derive(ToJs)]
    struct Position {
        x: f64,
        y: f64,
    }

    #[derive(ToJs)]
    struct Point {
        label: Option<String>,
        position: Nullable<Position>,
        color: Maybe<String>,
    }

    {
        let p1 = Point {
            label: Some("center".into()),
            position: Nullable::NonNull(Position { x: -3.2, y: 7.4 }),
            color: Maybe::Defined("red".into()),
        };

        let js = p1.to_js();

        assert!(js.is_object());
        assert!(js.is_truthy());

        let expect = jsv!({
            label: "center",
            position: jsv!({ x: -3.2, y: 7.4 }),
            color: "red",
        });

        assert!(js_test::deep_equal(&js, &expect));
    }

    {
        let p2 = Point {
            label: None,
            position: Nullable::Null,
            color: Maybe::Undefined,
        };

        let js = p2.to_js();

        assert!(js.is_object());
        assert!(js.is_truthy());

        let expect = jsv!({
            position: jsv!(null),
            color: jsv!(undefined),
        });

        assert!(js_test::deep_equal(&js, &expect));
    }
}

#[wasm_bindgen_test]
fn object_with_js_cast_prop() {
    #[derive(ToJs)]
    struct Position {
        x: js_sys::Number,
        date1: WrapJsCast<js_sys::Date>,
        date2: Nullable<WrapJsCast<js_sys::Date>>,
    }

    let js = Position {
        x: js_sys::Number::from(1),
        date1: WrapJsCast(js_sys::Date::new(&"2021-01-01T00:00:00+0800".into())),
        date2: Nullable::wrap_js_cast(js_sys::Date::new(&"2021-01-02T00:00:00+0800".into())),
    }
    .to_js();

    let expected = jsv!({
        x: 1,
        date1: &js_sys::Date::new(&"2021-01-01T00:00:00+0800".into()),
        date2: &js_sys::Date::new(&"2021-01-02T00:00:00+0800".into()),
    });

    assert!(deep_equal(&js, &expected));
}

#[wasm_bindgen_test]
fn tuple_struct() {
    #[derive(ToJs)]
    struct EmptyArray();

    assert!(deep_equal(&EmptyArray().to_js(), &jsv!([])));

    #[derive(ToJs)]
    #[convert_js(new_type_as_tuple)]
    struct Array1(String);

    assert!(deep_equal(
        &Array1("my_str".into()).to_js(),
        &jsv!(["my_str"])
    ));

    #[derive(ToJs)]
    struct Array2(bool, i32);
    assert!(deep_equal(&Array2(false, 1).to_js(), &jsv!([false, 1])));

    #[derive(ToJs)]
    struct Array5(bool, i32, Option<i32>, Nullable<i32>, ());
    assert!(deep_equal(
        &Array5(true, -4, None, Nullable::Null, ()).to_js(),
        &jsv!([true, -4, jsv!(undefined), jsv!(null), jsv!(undefined)]),
    ));

    #[derive(ToJs)]
    struct Array6(f64, f64, f64, f64, f64, f64);
    assert!(deep_equal(
        &Array6(0.0, 1.0, -1.0, f64::NAN, f64::NAN, f64::NAN).to_js(),
        &jsv!([0, 1, -1, f64::NAN, f64::NAN, f64::NAN]),
    ));
}

#[wasm_bindgen_test]
fn new_type_struct() {
    #[derive(ToJs)]
    struct Wrap(js_sys::Array);

    let arr = js_sys::Array::of1(&"my_value".into());

    assert_eq!(JsValue::from(&arr), Wrap(arr).to_js());
}
