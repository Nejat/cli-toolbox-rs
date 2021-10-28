# Examples

## CLI Reporting

Run the `report!` macro example to see the output behavior of the different levels of verbosity.

The verbosity level argument is optional and will default to `quite` if omitted.

Use the following command from the project root folder to run the example

* verbose
```shell
cargo run --release --example cli-reporting --features="report" -- verbose
```

* terse
```shell
cargo run --release --example cli-reporting --features="report" -- terse
```

* quite
```shell
cargo run --release --example cli-reporting --features="report"
```
```shell
cargo run --release --example cli-reporting --features="report" -- quite
```
