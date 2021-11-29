use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type ObjectConstructor;

    #[wasm_bindgen(method, structural, getter)]
    fn prototype(this: &ObjectConstructor) -> js_sys::Object;

    static Object: ObjectConstructor;
}

fn impl_is_empty_object(obj: &js_sys::Object) -> bool {
    let keys = js_sys::Reflect::own_keys(&obj).unwrap();
    keys.length() == 0 && {
        let json = js_sys::JSON::stringify(&obj);
        json == Ok(js_sys::JsString::from("{}"))
    }
}

pub fn is_empty_object_with_object_proto(obj: &js_sys::Object) -> bool {
    !obj.is_null()
        && {
            let proto = js_sys::Reflect::get_prototype_of(obj.as_ref()).unwrap();
            proto == (&*Object).prototype()
        }
        && impl_is_empty_object(obj)
}

pub fn is_empty_object_with_null_proto(obj: &js_sys::Object) -> bool {
    !obj.is_null()
        && {
            let proto = js_sys::Reflect::get_prototype_of(obj.as_ref()).unwrap();
            proto.is_null()
        }
        && impl_is_empty_object(obj)
}

pub fn is_empty_object(obj: &js_sys::Object) -> bool {
    is_empty_object_with_object_proto(obj) || is_empty_object_with_null_proto(obj)
}
