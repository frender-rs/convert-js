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

#[wasm_bindgen_test]
fn struct_rename() {
    #[derive(ToJs)]
    #[convert_js(rename_all = "camelCase")]
    struct MyObj {
        value: &'static str,
        my_data: i32,
        #[convert_js(rename(rule = "PascalCase"))]
        another_data: i32,
        #[convert_js(rename = "type")]
        kind: &'static str,
    }

    let js = MyObj {
        value: "my_value",
        my_data: 1,
        another_data: 0,
        kind: "obj",
    }
    .to_js();
    let expected = jsv!({
        value: "my_value",
        myData: 1,
        AnotherData: 0,
        type: "obj",
    });
    assert!(deep_equal(&js, &expected));
}

#[wasm_bindgen_test]
fn enum_union_literals() {
    #[derive(ToJs)]
    #[convert_js(union)]
    enum MyUnion {
        V1,
    }

    let v1 = MyUnion::V1;
    assert_eq!(v1.to_js(), "V1");

    #[derive(ToJs)]
    #[convert_js(union, rename_all = "SCREAMING_SNAKE_CASE")]
    enum MyUnionRenameAll {
        MyValueA,
        MyValueB,
    }

    assert_eq!(MyUnionRenameAll::MyValueA.to_js(), "MY_VALUE_A");
    assert_eq!(MyUnionRenameAll::MyValueB.to_js(), "MY_VALUE_B");

    #[derive(ToJs)]
    #[convert_js(union)]
    enum MyUnionRenameVariant {
        MyValueA,
        #[convert_js(rename(rule = "camelCase"))]
        MyValueB,
        #[convert_js(rename = "c")]
        MyValueC,
    }

    assert_eq!(MyUnionRenameVariant::MyValueA.to_js(), "MyValueA");
    assert_eq!(MyUnionRenameVariant::MyValueB.to_js(), "myValueB");
    assert_eq!(MyUnionRenameVariant::MyValueC.to_js(), "c");

    #[derive(ToJs)]
    #[convert_js(union, rename_all = "kebab-case")]
    enum MyUnionRenameMixed {
        MyValueA,
        #[convert_js(rename(rule = "PascalCase"))]
        MyValueB,
        MyValueC,
        #[convert_js(rename = "my value d")]
        MyValueD,
    }

    assert_eq!(MyUnionRenameMixed::MyValueA.to_js(), "my-value-a");
    assert_eq!(MyUnionRenameMixed::MyValueB.to_js(), "MyValueB");
    assert_eq!(MyUnionRenameMixed::MyValueC.to_js(), "my-value-c");
    assert_eq!(MyUnionRenameMixed::MyValueD.to_js(), "my value d");
}

#[wasm_bindgen_test]
fn enum_union() {
    #[derive(ToJs)]
    #[convert_js(union)]
    enum MyUnion {
        MyNewType(bool),
        LitA,
        LitB,
        #[convert_js(new_type_as_tuple)]
        UnaryTuple(i32),
        BinaryTuple(f64, bool),
        ObjectA {
            a: u32,
            b: String,
        },
    }

    assert_eq!(MyUnion::MyNewType(true).to_js(), true);
    assert_eq!(MyUnion::MyNewType(false).to_js(), false);
    assert_eq!(MyUnion::LitA.to_js(), "LitA");
    assert_eq!(MyUnion::LitB.to_js(), "LitB");
    assert!(deep_equal(&MyUnion::UnaryTuple(9).to_js(), &jsv!([9])));
    assert!(deep_equal(
        &MyUnion::BinaryTuple(-3.0, true).to_js(),
        &jsv!([-3.0, true]),
    ));
    assert!(deep_equal(
        &MyUnion::ObjectA {
            a: 1,
            b: "".to_owned()
        }
        .to_js(),
        jsv!({ a: 1, b: "" }).as_ref(),
    ));
}

#[wasm_bindgen_test]
fn enum_union_rename_all() {
    #[derive(ToJs)]
    #[convert_js(union, rename_all = "kebab-case")]
    enum MyUnion {
        MyNewType(bool),
        LitA,
        #[convert_js(rename = "b")]
        LitB,
        #[convert_js(rename(rule = "snake_case"))]
        LitC,
        #[convert_js(new_type_as_tuple)]
        UnaryTuple(i32),
        BinaryTuple(f64, bool),

        /// will inherit rename_all from enum
        ObjectA {
            my_a: u32,
            my_b: String,
        },
        #[convert_js(rename_all = "camelCase")]
        ObjectB {
            my_a: u32,
            my_b: String,
        },
    }

    assert_eq!(MyUnion::MyNewType(true).to_js(), true);
    assert_eq!(MyUnion::MyNewType(false).to_js(), false);
    assert_eq!(MyUnion::LitA.to_js(), "lit-a");
    assert_eq!(MyUnion::LitB.to_js(), "b");
    assert_eq!(MyUnion::LitC.to_js(), "lit_c");
    assert!(deep_equal(&MyUnion::UnaryTuple(9).to_js(), &jsv!([9])));
    assert!(deep_equal(
        &MyUnion::BinaryTuple(-3.0, true).to_js(),
        &jsv!([-3.0, true]),
    ));
    assert!(deep_equal(
        &MyUnion::ObjectA {
            my_a: 1,
            my_b: "my_value".to_owned()
        }
        .to_js(),
        jsv!({ "my-a": 1, "my-b": "my_value" }).as_ref(),
    ));
    assert!(deep_equal(
        &MyUnion::ObjectB {
            my_a: 1,
            my_b: "my value".to_owned()
        }
        .to_js(),
        jsv!({ "myA": 1, "myB": "my value" }).as_ref(),
    ));
}

#[wasm_bindgen_test]
fn enum_union_example_height() {
    #[derive(ToJs)]
    #[convert_js(union, rename_all = "camelCase")]
    enum Height {
        Unset,
        Auto,
        Number(f64),
        String(String),
    }

    assert_eq!(Height::Unset.to_js(), "unset");
    assert_eq!(Height::Auto.to_js(), "auto");
    assert_eq!(Height::Number(1.0).to_js(), 1.0);
    assert_eq!(Height::String("100%".to_owned()).to_js(), "100%");
}
