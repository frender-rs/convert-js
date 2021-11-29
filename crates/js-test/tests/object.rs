use wasm_bindgen::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen(inline_js = r#"
function create_non_empty_objects() {
    class MyObj {};
    return [
        null,
        { k: undefined },
        { k: 0 },
        { [Symbol("some_symbol")]: 0 },
        [],
        new MyObj(),
        create_object_with_non_enumerable_prop(),
    ];
}
function create_empty_object() { return {} }
function create_empty_object_with_null_proto() { return Object.create(null) }

function create_object_with_non_enumerable_prop() {
    const obj = {};
    Object.defineProperty(obj, 'key', { value: "some_value" });
    return obj;
}

module.exports = {
    create_non_empty_objects,
    create_empty_object,
    create_empty_object_with_null_proto,   
}
"#)]
extern "C" {
    fn create_non_empty_objects() -> Box<[js_sys::Object]>;
    fn create_empty_object() -> js_sys::Object;
    fn create_empty_object_with_null_proto() -> js_sys::Object;
}

#[wasm_bindgen_test]
fn object_is_empty() {
    use js_test::object::*;
    {
        let obj = create_empty_object();

        assert!(is_empty_object(&obj));
        assert!(is_empty_object_with_object_proto(&obj));

        assert!(!is_empty_object_with_null_proto(&obj));
    }

    {
        let obj = create_empty_object_with_null_proto();

        assert!(is_empty_object(&obj));
        assert!(is_empty_object_with_null_proto(&obj));

        assert!(!is_empty_object_with_object_proto(&obj));
    }

    {
        let objects = create_non_empty_objects();

        for obj in objects.into_iter() {
            assert!(!is_empty_object(&obj));
            assert!(!is_empty_object_with_null_proto(&obj));
            assert!(!is_empty_object_with_object_proto(&obj));
        }
    }
}
