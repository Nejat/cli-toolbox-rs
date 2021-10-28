use std::env;
use std::str::FromStr;

use verbosity::Verbosity;

use cli_toolbox::reportln;

fn main() {
    let level = Verbosity::from_str(&env::args().last().unwrap_or_else(String::new))
        .unwrap_or(Verbosity::Quite);

    // this will never print
    reportln! { "setting verbosity to {}", level }

    // this can only be set once, level is quite until it's set
    level.set_as_global();

    reportln! { "\nset verbosity to {}", level }

    // basic terse output
    reportln! { "terse output example" }

    // are you getting bored of 42 yet!? ... Douglas Adam Rulez!!!
    reportln! { @verbose "verbose formatted output example; {}", 42 }

    // terse or verbose error message
    reportln! {
        @err-terse   "some error message";
        @err-verbose "some more detailed error message";
    }

    reportln! { @verbose "so long and thanks for all the fish!\n" }
}
