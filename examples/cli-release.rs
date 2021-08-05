use cli_toolbox::*;

fn main() {
    Verbosity::Terse.set_as_global();

    report! { "example: starting" }

    debug! { "you are here <==" }

    let value;

    release! { value = 42 }

    debug! {{ value = 0 }}

    report! { "example: result = {}", value }

    debug! { "~ this example is so contrived ~" }

    report! { "example: ending" }
}