# Examples

## CLI Reporting

Run the `report!` macro example to see the output behavior of the different levels of verbosity.

The verbosity level argument is optional and will default to `quite` if omitted.

Use the following commands from the project root folder to run the examples

* __verbose__
```shell
cargo run --release --example cli-reporting --features="report" -- verbose
```

* __terse__
```shell
cargo run --release --example cli-reporting --features="report" -- terse
```

* __quite__
```shell
cargo run --release --example cli-reporting --features="report"
```
```shell
cargo run --release --example cli-reporting --features="report" -- quite
```

## CLI Debugging

Run the `debug!` macro example to see the output behavior of the different build optimizations

Use the following commands from the project root folder to run the examples

* __unoptimized__
```shell
cargo run --example cli-debugging --features="debug,report,verbosity"
```

* __optimized__
```shell
cargo run --release --example cli-debugging --features="debug,report,verbosity"
```
