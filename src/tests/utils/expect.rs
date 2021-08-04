/// testing macro for defining expected values for both release and debug
#[doc(hidden)]
#[macro_export]
macro_rules! expect {
    ($var:ident: $typ:ty => $rls:literal, $dgb:literal) => {
        cfg_if::cfg_if! {
            if #[cfg(not(debug_assertions))] {
                const $var: $typ = $rls;
            } else {
                const $var: $typ = $dgb;
            }
        }
    };
}
