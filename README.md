# cli-toolbox
Utility library for working with ```cli``` output ergonomically.

This is not a logging alternative, it's intended to produce output for end user consumption.

It handles three levels of verbosity that can be set dynamically at runtime:

* Quite - no output
* Terse - used to provide minimal user output
* Verbose - used to provide elaborated and/or additional user output 

### Output Macros

* `debug!` - conditionally compiled console debugging output - [`debug`]

* `report!` - conditional console output according to verbosity level - [`debug`|`release`]

\* _debug! is intended to be used during application development_

\* _all other debugging and telemetry output is most likely better served with a logging library_

### Conditional Code

* `eval!` - conditional code execution according to verbosity level - [`debug`|`release`]

* `release!` - conditional code execution according to verbosity level - [`release`]

## Resources 
* [Docs](https://docs.rs/cli-toolbox/0.4.0/cli_toolbox/) for more detailed information
* [Examples](https://github.com/Nejat/cli-toolbox-rs/tree/release/v0.4.0/examples) to see it in action

## Usage

Each macro is gated by a feature and no feature is mutually exclusive.

```toml
[dependencies]
cli-toolbox = { version = "0.4", features = ["debug", "eval", "release", "report"] }
```

## Roadmap

* [ ] ...

## Implemented
* [x] ```debug!``` macro
* [x] ```Verbosity``` reporting level functionality
* [x] ```report!``` macro
* [x] functionality separated by features
* [x] ```release!``` macro
* [x] ```eval!``` macro
