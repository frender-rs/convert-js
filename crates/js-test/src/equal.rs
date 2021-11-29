use js_sys::Reflect;
use wasm_bindgen::{JsCast, JsValue};

/// Checks v1 deeply equals to v2.
///
/// Doesn't support circular structures.
pub fn deep_equal(v1: &JsValue, v2: &JsValue) -> bool {
    if v1 == v2 || js_sys::Object::is(v1, v2) {
        return true;
    }

    let t1 = v1.js_typeof();
    let t2 = v2.js_typeof();

    if t1 != t2 {
        return false;
    };

    if t1 == "object" && !v1.is_null() && !v2.is_null() {
        let p1 = Reflect::get_prototype_of(v1).unwrap();
        let p2 = Reflect::get_prototype_of(v1).unwrap();

        p1 == p2 && {
            let a1 = v1.dyn_ref::<js_sys::Array>();
            let a2 = v2.dyn_ref::<js_sys::Array>();

            match (a1, a2) {
                (Some(a1), Some(a2)) => {
                    a1.length() == a2.length()
                        && a1.every(&mut |v, i, _| deep_equal(&v, &a2.get(i)))
                }
                (None, None) => {
                    let o1 = v1.dyn_ref::<js_sys::Object>();
                    let o2 = v2.dyn_ref::<js_sys::Object>();
                    match (o1, o2) {
                        (Some(o1), Some(o2)) => {
                            let d1 = o1.dyn_ref::<js_sys::Date>();
                            let d2 = o2.dyn_ref::<js_sys::Date>();

                            match (d1, d2) {
                                (Some(d1), Some(d2)) => return d1.get_time() == d2.get_time(),
                                _ => {}
                            }

                            let keys1 = js_sys::Reflect::own_keys(&o1).unwrap();
                            let keys2 =
                                js_sys::Set::new(js_sys::Reflect::own_keys(&o2).unwrap().as_ref());

                            keys1.for_each(&mut |v, _, _| {
                                keys2.delete(&v);
                            });

                            keys2.size() == 0
                                && keys1.every(&mut |k, _, _| {
                                    let v1 = js_sys::Reflect::get(&o1, &k).unwrap();
                                    let v2 = js_sys::Reflect::get(&o2, &k).unwrap();
                                    deep_equal(&v1, &v2)
                                })
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
    } else {
        false
    }
}
