use wasm_bindgen::{JsCast, JsValue};

pub trait ToJs {
    fn to_js(&self) -> JsValue;

    /// return `None` to leave property unset
    ///
    /// return `Some(value)` to set property
    fn to_js_property_value(&self) -> Option<JsValue> {
        Some(self.to_js())
    }
}

/// # Convert as js
///
/// `Some(value)` => `value`
///
/// `None`        => `undefined`
///
/// # Convert as js property value
///
/// `Some(value)` => property set to `value`
///
/// `None`        => property unset
impl<T> ToJs for Option<T>
where
    T: ToJs,
{
    fn to_js(&self) -> JsValue {
        if let Some(v) = self {
            v.to_js()
        } else {
            JsValue::UNDEFINED
        }
    }

    fn to_js_property_value(&self) -> Option<JsValue> {
        self.as_ref().map(|v| v.to_js())
    }
}

macro_rules! impl_to_js {
    (deref_copy: $($n:ty)*) => ($(
        impl ToJs for $n {
            #[inline]
            fn to_js(&self) -> JsValue {
                JsValue::from(*self)
            }
        }
    )*);
    (into: $($n:ty)*) => ($(
        impl ToJs for $n {
            #[inline]
            fn to_js(&self) -> JsValue {
                JsValue::from(self)
            }
        }
    )*);
}

impl_to_js! {
    into:
    String
    js_sys::Object
    js_sys::Array
    js_sys::JsString
    js_sys::Number
    js_sys::BigInt
    js_sys::Boolean
    js_sys::Function
}

impl_to_js! {
    deref_copy:
    &str
    // numbers https://docs.rs/wasm-bindgen/0.2.78/src/wasm_bindgen/lib.rs.html#849
    i8 u8 i16 u16 i32 u32 f32 f64
    // big_numbers https://docs.rs/wasm-bindgen/0.2.78/src/wasm_bindgen/lib.rs.html#869
    i64 u64 i128 u128 isize usize
    bool
}

impl<T: JsCast> ToJs for &T {
    fn to_js(&self) -> JsValue {
        JsValue::from(*self)
    }
}

macro_rules! impl_js_for_iter {
    ($($t:tt)+) => {
        $($t)+ {
            #[inline]
            fn to_js(&self) -> JsValue {
                js_sys::Array::from_iter(self.iter().map(|v| v.to_js())).into()
            }
        }
    };
}

impl_js_for_iter! { impl<T: ToJs> ToJs for Vec<T> }
impl_js_for_iter! { impl<T: ToJs> ToJs for &[T] }
impl_js_for_iter! { impl<N: ToJs, const S: usize> ToJs for [N; S] }

macro_rules! impl_js_for_tuple {
    (@impl $arr_method:ident ( $($t:ident),+ $(,)? )) => {
        impl<$($t: ToJs),+> ToJs for ($($t),+ ,) {
            #[inline]
            fn to_js(&self) -> JsValue {
                #![allow(non_snake_case)]
                let ($($t),+ ,) = self;
                js_sys::Array::$arr_method(
                    $(&$t.to_js()),+
                ).into()
            }
        }
    };
    (@impl ( $($t:ident),+ $(,)? )) => {
        impl<$($t: ToJs),+> ToJs for ($($t),+ ,) {
            #[inline]
            fn to_js(&self) -> JsValue {
                #![allow(non_snake_case)]
                let ($($t),+ ,) = self;
                js_sys::Array::from_iter([
                    $($t.to_js()),+
                ]).into()
            }
        }
    };
    ( $( $($arr_method:ident)? ( $($t:ident),+ $(,)? ) )* ) => {
        $(
            impl_js_for_tuple! { @impl $($arr_method)? ($($t),+ ,) }
        )*
    };
}

impl_js_for_tuple! {
    of1(T0,)
    of2(T0,T1)
    of3(T0,T1,T2)
    of4(T0,T1,T2,T3)
    of5(T0,T1,T2,T3,T4)
    (T0,T1,T2,T3,T4,T5)
    (T0,T1,T2,T3,T4,T5,T6)
    (T0,T1,T2,T3,T4,T5,T6,T7)
    (T0,T1,T2,T3,T4,T5,T6,T7,T8)
    (T0,T1,T2,T3,T4,T5,T6,T7,T8,T9)
}

impl<T: ToJs> ToJs for Box<T> {
    #[inline]
    fn to_js(&self) -> JsValue {
        self.as_ref().to_js()
    }
}

impl ToJs for () {
    fn to_js(&self) -> JsValue {
        JsValue::UNDEFINED
    }
}

impl ToJs for JsValue {
    fn to_js(&self) -> JsValue {
        self.clone()
    }
}

impl<T: ?Sized> ToJs for wasm_bindgen::prelude::Closure<T> {
    /// Note: unlike [`Closure::into_js_value`]
    /// after calling `closure.to_js`,
    /// the closure is not forgotten by rust memory.
    /// You should make sure it lives long enough to be called in js.
    ///
    /// [`Closure::into_js_value`]: https://docs.rs/wasm-bindgen/0.2.78/wasm_bindgen/closure/struct.Closure.html#method.into_js_value
    fn to_js(&self) -> JsValue {
        AsRef::<JsValue>::as_ref(&self).clone()
    }
}
