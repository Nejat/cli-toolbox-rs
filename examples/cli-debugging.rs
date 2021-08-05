use cli_toolbox::debug;
use cli_toolbox::report;
use cli_toolbox::Verbosity;

#[cfg(debug_assertions)]
const THE_ULTIMATE_ANSWER: u32 = 42;

fn main() {
    Verbosity::Terse.set_as_global();

    report! { "\nstarting cli-debugging example" }

    // this is a formatted stdout debugging example
    debug! {
        "this is an example of debugging output and it's formatted value {}",
        THE_ULTIMATE_ANSWER
    }

    report! { "trying answer 41" }

    // this is a conditional debugging example that's an err
    debug! {
        check_answer(41) =>
            _OK "genius! apple has a bartending job for you!";
            ERR "we have a problem with your answer";
    }

    report! { "trying answer 42, don't get this wrong" }

    // this is a conditional debugging example that's ok
    debug! {
        check_answer(42) =>
            _OK "genius! apple has a bartending job for you!";
            ERR "we have a problem with your answer";
    }

    // this is a stderr debugging example
    debug! { ERR "tantrum thrown" }

    report! { "cli-debugging example finished\n" }
}

#[cfg(debug_assertions)]
fn check_answer(proposed_answer: u32) -> Result<(), String> {
    if proposed_answer == THE_ULTIMATE_ANSWER {
        Ok(())
    } else {
        Err(String::from("i think not!"))
    }
}
