#![allow(unused_mut)] // for demonstration purposes

use verbosity::Verbosity;

use cli_toolbox::{debug, debugln, release, reportln};

fn main() {
    Verbosity::Terse.set_as_global();

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