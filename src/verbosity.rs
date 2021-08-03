//! Global verbosity level, used for reporting
use std::{
    fmt::{self, Display, Formatter}, str::FromStr,
    sync::{Arc, atomic::{AtomicBool, Ordering}},
};

use lazy_static::lazy_static;
use parking_lot::RwLock;

/// Output verbosity level <`verbose`|`terse`|`quite`>
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Verbosity {
    /// No output option
    Quite = 0,
    /// Minimal reporting option
    Terse = 1,
    /// Extended reporting option
    Verbose = 2,
}

impl Display for Verbosity {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Terse => fmt.write_str("terse"),
            Self::Verbose => fmt.write_str("verbose"),
            Self::Quite => fmt.write_str("quite"),
        }
    }
}

impl FromStr for Verbosity {
    type Err = String;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        match source {
            "terse" => Ok(Self::Terse),
            "verbose" => Ok(Self::Verbose),
            "quite" => Ok(Self::Quite),
            _ => Err(format!("{:?} is not a valid verbosity", source))
        }
    }
}

lazy_static! {
    static ref REPORTING: Arc<RwLock<Verbosity>> = Arc::new(RwLock::new(Verbosity::Terse));
    static ref REPORTING_SET: AtomicBool = AtomicBool::new(false);
}

impl Verbosity {
    /// Gets the global [`Verbosity`] level
    ///
    /// ```rust
    /// # use cli_toolbox::verbosity::Verbosity;
    /// Verbosity::Verbose.set_as_global();
    ///
    /// assert_eq!(Verbosity::level(), Verbosity::Verbose);
    /// ```
    #[must_use]
    #[inline]
    pub fn level() -> Self {
        *REPORTING.read()
    }

    /// Sets the instance of a [`Verbosity`] level as the global level
    ///
    /// ```rust
    /// # use cli_toolbox::verbosity::Verbosity;
    /// Verbosity::Verbose.set_as_global();
    ///
    /// assert_eq!(Verbosity::level(), Verbosity::Verbose);
    ///
    /// Verbosity::Quite.set_as_global();
    ///
    /// assert_ne!(Verbosity::level(), Verbosity::Quite);
    /// assert_eq!(Verbosity::level(), Verbosity::Verbose);
    /// ```
    ///
    /// \* _this can only be set once, all other calls will be ignored_
    ///
    /// [`Verbosity`]: crate::verbosity::Verbosity
    pub fn set_as_global(self) {
        let set = match REPORTING_SET.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(set) | Err(set) => set
        };

        if !set {
            *REPORTING.write() = self;
        }
    }
}
