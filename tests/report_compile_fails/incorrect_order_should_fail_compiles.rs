use cli_toolbox::{report, reportln};

fn main() {
    report! {
        @verbose "verbose message: {}", 42;
        @terse "terse message: {}", 42
    }

    report! {
        @verbose "verbose message: {}", 42;
        @err-terse "error terse message: {}", -42
    }

    report! {
        @err-verbose "error verbose message: {}", -42;
        @terse "terse message: {}", 42
    }

    report! {
        @err-verbose "error verbose message: {}", -42;
        @err-terse "error terse message: {}", -42
    }

    reportln! {
        @verbose "verbose message: {}", 42;
        @terse "terse message: {}", 42
    }

    reportln! {
        @verbose "verbose message: {}", 42;
        @err-terse "error terse message: {}", -42
    }

    reportln! {
        @err-verbose "error verbose message: {}", -42;
        @terse "terse message: {}", 42
    }

    reportln! {
        @err-verbose "error verbose message: {}", -42;
        @err-terse "error terse message: {}", -42
    }
}