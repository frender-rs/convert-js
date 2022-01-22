use wasm_bindgen::{JsCast, JsValue};

use crate::{FromJs, ToJs, WrapJsCast};

macro_rules! def_option_like {
    (
        $name:ident {
            None = $name_none:ident = $js_none_doc:literal $js_none:expr,
            condition = |$from_js_ident:ident| $js_is_none:expr,
            Some = $name_some:ident $(,)?
        }
    ) => {
        #[doc = concat!("Corresponding to `T | ", $js_none_doc, "`")]
        ///
        /// # Convert as js
        ///
        #[doc = concat!("`", stringify!($name_none), "` => `", $js_none_doc, "`")]
        ///
        #[doc = concat!("`", stringify!($name_some), "(value)` => `value`")]
        ///
        /// # Convert as js property value
        ///
        /// Property is always set
        #[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
        pub enum $name<T> {
            $name_none,
            $name_some(T),
        }

        impl<T> $name<T> {
            #[inline]
            pub fn from_option(v: Option<T>) -> Self {
                if let Some(v) = v {
                    Self::$name_some(v)
                } else {
                    Self::$name_none
                }
            }

            #[inline]
            pub fn into_option(self) -> Option<T> {
                match self {
                    Self::$name_none => None,
                    Self::$name_some(v) => Some(v),
                }
            }

            #[inline]
            pub fn as_ref(&self) -> $name<&T> {
                match self {
                    Self::$name_none => $name::$name_none,
                    Self::$name_some(v) => $name::$name_some(v),
                }
            }
        }

        impl<T: Clone> Clone for $name<T> {
            #[inline]
            fn clone(&self) -> Self {
                match self {
                    Self::$name_some(x) => Self::$name_some(x.clone()),
                    Self::$name_none => Self::$name_none,
                }
            }

            #[inline]
            fn clone_from(&mut self, source: &Self) {
                match (self, source) {
                    (Self::$name_some(to), Self::$name_some(from)) => to.clone_from(from),
                    (to, from) => *to = from.clone(),
                }
            }
        }

        impl<T> Default for $name<T> {
            #[doc = concat!("Returns [`", stringify!($name_none), "`][", stringify!($name), "::", stringify!($name_none), "]")]
            #[inline]
            fn default() -> $name<T> {
                Self::$name_none
            }
        }

        impl<T> From<Option<T>> for $name<T> {
            #[inline]
            fn from(v: Option<T>) -> Self {
                Self::from_option(v)
            }
        }

        impl<T> Into<Option<T>> for $name<T> {
            #[inline]
            fn into(self) -> Option<T> {
                self.into_option()
            }
        }

        impl<T: ToJs> ToJs for $name<T> {
            fn to_js(&self) -> wasm_bindgen::JsValue {
                match self {
                    Self::$name_none => $js_none,
                    Self::$name_some(v) => v.to_js(),
                }
            }
        }

        impl<T: FromJs> FromJs for $name<T> {
            type Error = T::Error;
            fn from_js($from_js_ident: wasm_bindgen::JsValue) -> Result<Self, Self::Error> {
                if $js_is_none {
                    Ok(Self::$name_none)
                } else {
                    T::from_js($from_js_ident).map(Self::$name_some)
                }
            }
        }

        impl<T: JsCast> $name<WrapJsCast<T>> {
            pub fn wrap_js_cast(v: T) -> Self {
                Self::$name_some(WrapJsCast(v))
            }
        }
    };
}

def_option_like! {
    Nullable {
        None = Null = "null" JsValue::NULL,
        condition = |v| v.is_null(),
        Some = NonNull,
    }
}

def_option_like! {
    Maybe {
        None = Undefined = "undefined" JsValue::UNDEFINED,
        condition = |v| v.is_undefined(),
        Some = Defined,
    }
}
