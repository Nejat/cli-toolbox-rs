/// testing macro for defining actual variables for both release and debug
#[doc(hidden)]
#[macro_export]
macro_rules! actual {
    ($var:ident: $typ:ty) => {
        cfg_if::cfg_if! {
            if #[cfg(not(debug_assertions))] {
                let $var: $typ = Default::default();
            } else {
                let $var: $typ;
            }
        }
    };
    ($var:ident = $exp:expr) => {
        cfg_if::cfg_if! {
            if #[cfg(not(debug_assertions))] {
                let $var = $exp;
            } else {
                let mut $var = $exp;
            }
        }
    }
}