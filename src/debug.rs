/// Macro to output statements or execute expressions in debug builds
#[doc(hidden)]
#[macro_export]
macro_rules! debug {
    // ===============================================================================
    // evaluate block
    ($debug:block) => {
        #[cfg(debug_assertions)]
        $debug
    };
    // ===============================================================================
    // output message
    ($debug:expr) => {
        #[cfg(debug_assertions)]
        debug! { @ $debug }
    };
    // output formatted message
    ($debug:expr, $($val:expr),*) => {
        #[cfg(debug_assertions)]
        debug! { @ $debug, $($val),* }
    };
    // ===============================================================================
    // output error message
    (ERR $debug:expr) => {
        #[cfg(debug_assertions)]
        debug! { @ERR $debug }
    };
    // output formatted error message
    (ERR $debug:expr, $($val:expr),*) => {
        #[cfg(debug_assertions)]
        debug! { @ERR $debug, $($val),* }
    };
    // ===============================================================================
    // evaluates block if result is ok
    ($result:expr => OK $ok:ident $debug:block) => {
        #[cfg(debug_assertions)]
        if let Ok($ok) = $result { $debug }
    };
    // evaluates block if result is ok, discards ok
    ($result:expr => OK $debug:block) => {
        #[cfg(debug_assertions)]
        if let Ok(_) = $result { $debug }
    };
    // ===============================================================================
    // evaluates block if result is err
    ($result:expr => ERR $err:ident $debug:block) => {
        #[cfg(debug_assertions)]
        if let Err($err) = $result { $debug }
    };
    // evaluates block if result is err, discards err
    ($result:expr => ERR $debug:block) => {
        #[cfg(debug_assertions)]
        if let Err(_) = $result { $debug }
    };
    // ===============================================================================
    // evaluates blocks based on result
    (
        $result:expr =>
            OK $ok:ident $succeeded:block
            ERR $err:ident $failed:block
    ) => {
        #[cfg(debug_assertions)]
        match $result {
            Ok($ok) => $succeeded
            Err($err) => $failed
        }
    };
    // evaluates blocks based on result, discards ok
    (
        $result:expr =>
            OK $succeeded:block
            ERR $err:ident $failed:block
    ) => {
        #[cfg(debug_assertions)]
        match $result {
            Ok(_) => $succeeded
            Err($err) => $failed
        }
    };
    // evaluates blocks based on result, discards err
    (
        $result:expr =>
            OK $ok:ident $succeeded:block
            ERR $failed:block
    ) => {
        #[cfg(debug_assertions)]
        match $result {
            Ok($ok) => $succeeded
            Err(_) => $failed
        }
    };
    // evaluates blocks based on result, discards results
    (
        $result:expr =>
            OK $succeeded:block
            ERR $failed:block
    ) => {
        #[cfg(debug_assertions)]
        match $result {
            Ok(_) => $succeeded
            Err(_) => $failed
        }
    };
    // ===============================================================================
    // output success message if result is ok
    ($result:expr => OK $debug:expr) => {
        #[cfg(debug_assertions)]
        if let Ok(ok) = $result { debug! { @ concat!($debug, ": {:?}"), ok } }
    };
    // output success message if result is ok, discards ok
    ($result:expr => _OK $debug:expr) => {
        #[cfg(debug_assertions)]
        if let Ok(_) = $result { debug! { @ $debug } }
    };
    // output formatted success message if result is ok
    ($result:expr => OK $debug:expr, $($val:expr),*) => {
        #[cfg(debug_assertions)]
        if let Ok(ok) = $result { debug! { @ concat!($debug, ": {:?}"), $($val),*, ok } }
    };
    // output formatted success message if result is ok, discards ok
    ($result:expr => _OK $debug:expr, $($val:expr),*) => {
        #[cfg(debug_assertions)]
        if let Ok(_) = $result { debug! { @ $debug, $($val),* } }
    };
    // ===============================================================================
    // output error message if result is err
    ($result:expr => ERR $debug:expr) => {
        #[cfg(debug_assertions)]
        if let Err(err) = $result { debug! { @ERR concat!($debug, ": {:?}"), err } }
    };
    // output error message if result is err, discards err
    ($result:expr => _ERR $debug:expr) => {
        #[cfg(debug_assertions)]
        if let Err(_) = $result { debug! { @ERR $debug } }
    };
    // output formatted error message if result is err
    ($result:expr => ERR $debug:expr, $($val:expr),*) => {
        #[cfg(debug_assertions)]
        if let Err(err) = $result { debug! { @ERR concat!($debug, ": {:?}"), $($val),*, err } }
    };
    // output formatted error message if result is err, discards err
    ($result:expr => _ERR $debug:expr, $($val:expr),*) => {
        #[cfg(debug_assertions)]
        if let Err(_) = $result { debug! { @ERR $debug, $($val),* } }
    };
    // ===============================================================================
    // outputs messages based on result
    (
        $result:expr =>
            OK $succeeded:expr;
            ERR $failed:expr;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(ok) => debug! { @ concat!($succeeded, ": {:?}"), ok },
            Err(err) => debug! { @ERR concat!($failed, ": {:?}"), err }
        }
    };
    // outputs messages based on result, discards ok
    (
        $result:expr =>
            _OK $succeeded:expr;
            ERR $failed:expr;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(_) => debug! { @ $succeeded },
            Err(err) => debug! { @ERR concat!($failed, ": {:?}"), err }
        }
    };
    // outputs messages based on result, discards err
    (
        $result:expr =>
            OK $succeeded:expr;
            _ERR $failed:expr;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(ok) => debug! { @ concat!($succeeded, ": {:?}"), ok },
            Err(_) => debug! { @ERR  $failed }
        }
    };
    // outputs messages based on result, discards results
    (
        $result:expr =>
            _OK $succeeded:expr;
            _ERR $failed:expr;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(_) => debug! { @ $succeeded },
            Err(_) => debug! { @ERR $failed }
        }
    };
    // ===============================================================================
    // outputs formatted messages based on result
    (
        $result:expr =>
            OK $succeeded:expr, $($ok_val:expr),*;
            ERR $failed:expr, $($err_val:expr),*;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(ok) => debug! { @ concat!($succeeded, ": {:?}"), $($ok_val),*, ok },
            Err(err) => debug! { @ERR concat!($failed, ": {:?}"), $($err_val),*, err }
        }
    };
    // outputs formatted messages based on result, discards ok
    (
        $result:expr =>
            _OK $succeeded:expr, $($ok_val:expr),*;
            ERR $failed:expr, $($err_val:expr),*;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(_) => debug! { @ $succeeded, $($ok_val),* },
            Err(err) => debug! { @ERR concat!($failed, ": {:?}"), $($err_val),*, err }
        }
    };
    // outputs formatted messages based on result, discards err
    (
        $result:expr =>
            OK $succeeded:expr, $($ok_val:expr),*;
            _ERR $failed:expr, $($err_val:expr),*;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(ok) => debug! { @ concat!($succeeded, ": {:?}"), $($ok_val),*, ok },
            Err(_) => debug! { @ERR $failed, $($err_val),* }
        }
    };
    // outputs formatted messages based on result, discards results
    (
        $result:expr =>
            _OK $succeeded:expr, $($ok_val:expr),*;
            _ERR $failed:expr, $($err_val:expr),*;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(_) => debug! { @ $succeeded, $($ok_val),* },
            Err(_) => debug! { @ERR $failed, $($err_val),* }
        }
    };
    // ===============================================================================
    // outputs formatted success message based on result
    (
        $result:expr =>
            OK $succeeded:expr, $($ok_val:expr),*;
            ERR $failed:expr;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(ok) => debug! { @ concat!($succeeded, ": {:?}"), $($ok_val),*, ok },
            Err(err) => debug! { @ERR concat!($failed, ": {:?}"), err }
        }
    };
    // outputs formatted success message based on result, discards ok
    (
        $result:expr =>
            _OK $succeeded:expr, $($ok_val:expr),*;
            ERR $failed:expr;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(_) => debug! { @ $succeeded, $($ok_val),* },
            Err(err) => debug! { @ERR concat!($failed, ": {:?}"), err }
        }
    };
    // outputs formatted success message based on result, discards err
    (
        $result:expr =>
            OK $succeeded:expr, $($ok_val:expr),*;
            _ERR $failed:expr;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(ok) => debug! { @ concat!($succeeded, ": {:?}"), $($ok_val),*, ok },
            Err(_) => debug! { @ERR $failed }
        }
    };
    // outputs formatted success message based on result, discards results
    (
        $result:expr =>
            _OK $succeeded:expr, $($ok_val:expr),*;
            _ERR $failed:expr;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(_) => debug! { @ $succeeded, $($ok_val),* },
            Err(_) => debug! { @ERR $failed }
        }
    };
    // ===============================================================================
    // outputs formatted error message based on result
    (
        $result:expr =>
            OK $succeeded:expr;
            ERR $failed:expr, $($err_val:expr),*;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(ok) => debug! { @ concat!($succeeded, ": {:?}"), ok },
            Err(err) => debug! { @ERR concat!($failed, ": {:?}"), $($err_val),*, err }
        }
    };
    // outputs formatted error message based on result, discards ok
    (
        $result:expr =>
            _OK $succeeded:expr;
            ERR $failed:expr, $($err_val:expr),*;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(_) => debug! { @ $succeeded },
            Err(err) => debug! { @ERR concat!($failed, ": {:?}"), $($err_val),*, err }
        }
    };
    // outputs formatted error message based on result, discards err
    (
        $result:expr =>
            OK $succeeded:expr;
            _ERR $failed:expr, $($err_val:expr),*;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(ok) => debug! { @ concat!($succeeded, ": {:?}"), ok },
            Err(_) => debug! { @ERR $failed, $($err_val),* }
        }
    };
    // outputs formatted error message based on result, discards results
    (
        $result:expr =>
            _OK $succeeded:expr;
            _ERR $failed:expr, $($err_val:expr),*;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(_) => debug! { @ $succeeded },
            Err(_) => debug! { @ERR $failed, $($err_val),* }
        }
    };
    // ===============================================================================
    // private output message
    (@ $debug:expr) => {
        println!(concat!("DEBUG: ", $debug));
    };
    // private output formatted message
    (@ $debug:expr, $($val:expr),*) => {
        println!(concat!("DEBUG: ", $debug), $($val),*);
    };
    // ===============================================================================
    // private output error message
    (@ERR $debug:expr) => {
        eprintln!(concat!("ERROR: ", $debug));
    };
    // private output formatted error message
    (@ERR $debug:expr, $($val:expr),*) => {
        eprintln!(concat!("ERROR: ", $debug), $($val),*);
    };
}