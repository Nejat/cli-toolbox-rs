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
* [Docs](https://docs.rs/cli-toolbox/0.8.1/cli_toolbox/) for more detailed information
* [Examples](https://github.com/Nejat/cli-toolbox-rs/tree/v0.8.1/examples) to see it in action

## Usage

Each macro is gated by a feature.

No feature is mutually exclusive and can be combined as needed.

* `debug!` macro

```toml
[dependencies]
cli-toolbox = { version = "0.8", features = ["debug"] }
```

* `eval!` macro

```toml
[dependencies]
cli-toolbox = { version = "0.8", features = ["eval"] }
verbosity = "0.1"
```

* `release!` macro

```toml
[dependencies]
cli-toolbox = { version = "0.8", features = ["release"] }
verbosity = "0.1"
```

* `report!` macro

```toml
[dependencies]
cli-toolbox = { version = "0.8", features = ["report"] }
verbosity = "0.1"
```

## Roadmap

* [ ] ...

## Implemented
* [x] ```debug!``` macro
* [x] ```eval!``` macro
* [x] ```release!``` macro
* [x] ```report!``` macro
