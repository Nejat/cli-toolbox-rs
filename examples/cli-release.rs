#![allow(unused_mut)] // for demonstration purposes

use std::env;
use std::str::FromStr;

use verbosity::Verbosity;

use cli_toolbox::{debug, debugln, release, reportln};

fn main() {
    let level = Verbosity::from_str(&env::args().last().unwrap_or_else(String::new))
        .unwrap_or(Verbosity::Quite);

    // this will never print
    reportln! { "setting verbosity to {}", level }

    // this can only be set once, level is quite until it's set
    level.set_as_global();

    reportln! { "example: starting" }

    debugln! { "you are here <==" }

    let mut value = 0;

    release! { value = 42 }

    // this will not evaluate based on the verbosity set for this example
    release! { @verbose value = 41 }

    debug! { assert_eq!(0, value); }

    reportln! { "example: result = {}", value }

    debugln! { "~ this example is so contrived ~" }

    reportln! { "example: ending" }
}