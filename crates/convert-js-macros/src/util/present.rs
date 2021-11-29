use darling::util::Flag;

pub trait IsPresent {
    fn is_present(&self) -> bool;
}

impl<T> IsPresent for Option<T> {
    fn is_present(&self) -> bool {
        self.is_some()
    }
}

impl IsPresent for Flag {
    fn is_present(&self) -> bool {
        self.is_some()
    }
}

pub fn ensure_not_present<P: IsPresent>(
    p: &P,
    name: &str,
    conflict_with: &str,
) -> Result<(), String> {
    if p.is_present() {
        Err(format!("`{}` cannot be used with {}", name, conflict_with))
    } else {
        Ok(())
    }
}

macro_rules! not_present {
    ($p:expr, $conflict_with:ident) => {
        $crate::util::ensure_not_present(&($p), stringify!($p), stringify!($conflict_with))
    };
    ($p:expr, $conflict_with:literal) => {
        $crate::util::ensure_not_present(&($p), stringify!($p), $conflict_with)
    };
    ($p:expr, struct) => {
        $crate::util::ensure_not_present(&($p), stringify!($p), "struct")
    };
    ($p:expr, enum) => {
        $crate::util::ensure_not_present(&($p), stringify!($p), "enum")
    };
}

pub(crate) use not_present;
