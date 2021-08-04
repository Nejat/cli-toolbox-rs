# Examples

## CLI Reporting

Run the `report!` macro example to see the output behavior of the different levels of verbosity.

The verbosity level argument is optional and will default to `quite` if omitted.

Use the following command from the project root folder to run the example

```shell
cargo run --release --example cli-reporting --features="debug,report" [-- <quite|terse|verbose>]
```

## CLI Debugging 

Run the `debug!` macro example to see how debugging output behaves.

Use the following command from the project root folder to run the example in debug build

```shell
cargo run --example cli-debugging --features="debug,report"
```

and then use the following command to run the example in release build to compare the output of both builds

```shell
cargo run --release --example cli-debugging --features="debug,report"
```
