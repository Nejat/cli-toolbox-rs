
For basic reporting, input consists of an optional intended verbosity level, defaulting to `terse` 
if it is not specifically provided. The remainder of the macro input is the same as the `std` 
library [`println!`] macro. An optional semicolon can terminate the message.

_\* all of the examples below also apply to the `report!` macro variant_

### Examples

* reports to `io::stdout` if `Verbosity` level is `terse` or `verbose` 
```no_compile
reportln! { @terse "some {} message to report", "important" }
```
* reports to `io::stdout` if `Verbosity` level is `verbose`
```no_compile
reportln! { @verbose "some {} message to report", "important" }
``` 
* reports to `io::stderr` if `Verbosity` level is `terse` or `verbose`
```no_compile
reportln! { @err-terse "some {} message to report", "important" }
```
* reports to `io::stderr` if `Verbosity` level is `verbose`
```no_compile
reportln! { @err-verbose "some {} message to report", "important" }
``` 

### _Separate Messages_

If you want to report a different messages based on specific verbosity levels you can provide two 
messages.

First the `terse` message, followed by a required semicolon separator and then by the second message. 

### Examples
* reports to `io::stdout` based on `Verbosity` level

```no_compile
reportln! {
    @terse "some basic message";
    @verbose "more verbose message"
}
```

* reports to `io::stderr` based on `Verbosity` level

```no_compile
reportln! {
    @err-terse "some basic message";
    @err-verbose "more verbose message"
}
```

* reports to `io::stdout` or `io::stderr` based on `Verbosity` level

```no_compile
reportln! {
    @terse "some basic message";
    @err-verbose "more verbose message"
}
```

```no_compile
reportln! {
    @err-terse "some basic message";
    @verbose "more verbose message"
}
```

_\* unlike the  basic variant above, here the `terse` message only outputs if the `Verbosity` level is
specifically `terse`, otherwise the second message is output if the level is `verbose`_

## Panics

Just like the [`println!`] macros used, by this macro, to write the output, this also panics if writing 
to `io::stdout` or `io::stderr` fails.

[`Verboisty`]: <https://crates.io/crates/verbosity>
[`println!`]: <https://doc.rust-lang.org/std/macro.println.html>