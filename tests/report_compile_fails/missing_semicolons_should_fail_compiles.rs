use cli_toolbox::{report, reportln};

fn main() {
    report! {
        @terse "terse message: {}", 42
        @verbose "verbose message: {}", 42
    }

    report! {
        @err-terse "error terse message: {}", -42
        @verbose "verbose message: {}", 42
    }

    report! {
        @terse "terse message: {}", 42
        @err-verbose "error verbose message: {}", -42
    }

    report! {
        @err-terse "error terse message: {}", -42
        @err-verbose "error verbose message: {}", -42
    }

    reportln! {
        @terse "terse message: {}", 42
        @verbose "verbose message: {}", 42
    }

    reportln! {
        @err-terse "error terse message: {}", -42
        @verbose "verbose message: {}", 42
    }

    reportln! {
        @terse "terse message: {}", 42
        @err-verbose "error verbose message: {}", -42
    }

    reportln! {
        @err-terse "error terse message: {}", -42
        @err-verbose "error verbose message: {}", -42
    }
}