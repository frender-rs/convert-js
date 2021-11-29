mod to_js;
pub use to_js::*;

mod option_like;
pub use option_like::*;

pub use Maybe::*;
pub use Nullable::*;

pub use js_sys;

#[path = "./internal/mod.rs"]
pub mod __internal;

#[macro_export]
macro_rules! jsv {
    (undefined) => {
        $crate::__internal::JsValue::UNDEFINED
    };
    (null) => {
        $crate::__internal::JsValue::NULL
    };
    ($v:literal) => {
        $crate::__internal::JsValue::from($v)
    };
    ({}) => {
        $crate::js_sys::Object::new()
    };
    ({ $( $k:tt $(: $v:expr)? ),+ $(,)? }) => {{
        let __obj = $crate::__internal::JsObject::new();
        $(
            $crate::jsv!(@impl set_prop_and_check
                __obj
                ($k)
                $(: $v)?
            );
        )+
        __obj.into_inner()
    }};
    (@impl set_prop_and_check $obj:ident ($k:tt) : $v:expr ) => {
        $obj.set_prop(
            &($crate::jsv!(@impl resolve_prop_key $k)),
            &($v),
        )
    };
    (@impl set_prop_and_check $obj:ident ($k:tt) ) => {
        $obj.set_prop(
            &($crate::jsv!(@impl resolve_prop_key $k)),
            &($k),
        )
    };
    (@impl resolve_prop_key $k:ident) => { stringify!($k) };
    (@impl resolve_prop_key $k:literal) => { $k };
    (@impl resolve_prop_key [$k:expr]) => { $crate::jsv!(# $k) };
    (@impl resolve_prop_key $k:tt) => {
        compile_error!(concat!("invalid syntax for js object property key: ", stringify!($k)))
    };
    ([]) => {
        $crate::js_sys::Array::new()
    };
    ([ $($arr_item:expr),+ $(,)? ]) => {
        $crate::js_sys::Array::from_iter([
            $(
                $crate::ToJs::to_js(&($arr_item))
            ),+
        ])
    };
}
