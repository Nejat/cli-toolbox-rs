Utility library for working with ```cli``` output ergonomically.

This is not a logging alternative, it's intended to produce output for end user consumption.

It handles three levels of verbosity that can be set dynamically at runtime:

* Quite - no output
* Terse - used to provide minimal user output
* Verbose - used to provide elaborated and/or additional user output 

Additionally, this library provides conditionally compiled debugging output intended to be used during application development.

All other debugging and telemetry output is most likely better served with a logging library.

## Usage

```toml
[dependencies]
cli-toolbox = "0.4.0"
```
# Roadmap
* [x] ```debug!``` macro
* [x] ```Verbosity``` reporting level functionality
* [x] ```report!``` macro
* [x] functionality separated by features

# Roadmap - ¿ Extra Credit ?
* [ ] handle ```Option<T>``` expressions in addition to ```Result<T,E>```
      conditional expressions

