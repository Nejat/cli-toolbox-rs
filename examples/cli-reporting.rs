// use std::env;
//
// use cli_toolbox::report;
// use cli_toolbox::Verbosity;
// use std::str::FromStr;

fn main() {
//     let level: Verbosity = env::args()
//         .map(|v| Verbosity::from_str(&v))
//         .filter(|v| v.is_ok())
//         .map(|v| v.unwrap())
//         .last()
//         .unwrap_or(Verbosity::Quite);
//
//     // this will never print
//     report! { "setting verbosity to {}", level }
//
//     // this can only be set once, level is quite until it's set
//     level.set_as_global();
//
//     report! { "\nset verbosity to {}", level }
//
//     // basic terse output
//     report! { "terse output example" }
//
//     // are getting bored of 42 yet!? ... Douglas Adam Rulez!!!
//     report! { VERBOSE "verbose formatted output example; {}", 42 }
//
//     // terse or verbose error message
//     report! {
//         ERR  "gratuitous error message\n";
//         ERRV "gratuitously more detailed error message";
//     }
//
//     report! { VERBOSE "so long and thanks for all the fish!\n" }
}
