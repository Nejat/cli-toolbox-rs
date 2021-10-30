#[cfg(debug_assertions)]
use macrofied_toolbox::result;
use verbosity::Verbosity;

use cli_toolbox::{debug, debugln};
use cli_toolbox::reportln;

#[cfg(debug_assertions)]
const THE_ULTIMATE_ANSWER: u32 = 42;

fn main() {
    Verbosity::Terse.set_as_global();

    reportln! { "\nstarting cli-debugging example" }

    // this is a formatted stdout debugging example
    debugln! {
        "\nDBG: this is an example of debugging output and it's formatted value {}",
        THE_ULTIMATE_ANSWER
    }

    // this is a debugging expression example
    debug! {
        result! {
            @when  check_answer(42);
            @ok    "DGB: genius! apple has a bartending job for you!"
            @error "DGB-ERR: we have a problem with your answer, please proceed over to that absolutely non-ominous room"
        }
    }

    reportln! { "\ncli-debugging example finished\n" }
}

#[cfg(debug_assertions)]
fn check_answer(proposed_answer: u32) -> Result<(), &'static str> {
    if proposed_answer == THE_ULTIMATE_ANSWER {
        Ok(())
    } else {
        Err("i think not!")
    }
}
