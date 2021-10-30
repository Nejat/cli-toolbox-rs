use verbosity::Verbosity;

use cli_toolbox::{debugln, eval, reportln};

fn main() {
    Verbosity::Terse.set_as_global();

    reportln! { "example: starting" }

    debugln! { "you are here <==" }

    let mut value = 0;

    eval! { @terse value = 42 }

    // this will not evaluate based on the verbosity set for this example
    eval! { @verbose value = 0 }

    reportln! { "example: result = {}", value }

    debugln! { "~ this example is so contrived ~" }

    reportln! { "example: ending" }
}