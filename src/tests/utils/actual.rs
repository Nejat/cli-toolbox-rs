/// testing macro for defining actual variables for both release and debug
#[doc(hidden)]
#[macro_export]
macro_rules! actual {
    // debug uninitialized actual variable
    ($var:ident: $typ:ty) => {
        cfg_if::cfg_if! {
            if #[cfg(not(debug_assertions))] {
                let $var: $typ = Default::default();
            } else {
                let $var: $typ;
            }
        }
    };
    // release uninitialized actual variable
    (@rls $var:ident: $typ:ty) => {
        cfg_if::cfg_if! {
            if #[cfg(not(debug_assertions))] {
                let $var: $typ;
            } else {
                let $var: $typ = Default::default();
            }
        }
    };
    // debug actual variable
    ($var:ident = $exp:expr) => {
        cfg_if::cfg_if! {
            if #[cfg(not(debug_assertions))] {
                let $var = $exp;
            } else {
                let mut $var = $exp;
            }
        }
    };
    // release actual variable
    (@rls $var:ident = $exp:expr) => {
        cfg_if::cfg_if! {
            if #[cfg(not(debug_assertions))] {
                let mut $var = $exp;
            } else {
                let $var = $exp;
            }
        }
    };
}
