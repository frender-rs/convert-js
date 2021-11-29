use wasm_bindgen::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen(inline_js = r#"
function values_should_deep_equal() {
    let sym = Symbol("some_symbol");

    class MyObj {
        constructor(value) {
            this.value = value;
        }
    }

    return [
        [undefined, undefined],
        ["my-value", "my-value"],
        [Symbol.iterator, Symbol.iterator],
        [123456n, 123456n],
        [1, 1.0],
        [0, -0],
        [NaN, NaN],
        [true, true],
        [false, false],
        [true, true],
        [null, null],
        [{}, {}],
        [{ k: 1 }, { k: 1 }],
        [{ k: [1, { v: 2 }]}, { k: [1, { v: 2 }]}],
        [{ k: { v: [false] }}, { k: { v: [false] }}],
        [{ [sym]: 0 }, { [sym]: 0 }],
        [new MyObj({k:"v"}), new MyObj({k:"v"})],
        [[], []],
        [[false, "1", 2], [false, "1", 2]],
        [[{k:1},{k2:2}], [{k:1},{k2:2}]],
    ];
}
function values_should_not_deep_equal() {
    class MyObj {
        constructor(value) {
            this.value = value;
        }
    }

    return [
        [undefined, null],
        ["my-value", "my-another-value"],
        [Symbol("some_sym"), Symbol("some_sym")],
        [123456n, 123456],
        [1, 1.1],
        [0, NaN],
        [true, false],
        [null, {}],
        [0, []],
        [-0, false],
        [{}, Object.create(null)],
        [{ k: 1 }, { k: 2 }],
        [{ k: 1 }, { k2: 1 }],
        [{ k: 1 }, {}],
        [{ k: [1, { v: 2 }]}, { k: [1]}],
        [{ k: { v: [] }}, { k: { v: [undefined] }}],
        [{ [Symbol("some_symbol")]: 0 }, { [Symbol("some_symbol")]: 0 }],
        [new MyObj({k:"v"}), {k:"v"}],
    ];
}
module.exports = {
    values_should_deep_equal,
    values_should_not_deep_equal,
}
"#)]
extern "C" {
    fn values_should_deep_equal() -> Box<[js_sys::Array]>;
    fn values_should_not_deep_equal() -> Box<[js_sys::Array]>;
}

#[wasm_bindgen_test]
fn test_deep_equal() {
    use js_test::deep_equal;

    let tuples = values_should_deep_equal();

    for tuple in tuples.into_iter() {
        let v1 = tuple.get(0);
        let v2 = tuple.get(1);

        assert!(deep_equal(&v1, &v2));
    }
}

#[wasm_bindgen_test]
fn test_not_deep_equal() {
    use js_test::deep_equal;

    let tuples = values_should_not_deep_equal();

    for tuple in tuples.into_iter() {
        let v1 = tuple.get(0);
        let v2 = tuple.get(1);

        assert!(!deep_equal(&v1, &v2));
    }
}
