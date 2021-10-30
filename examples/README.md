# Examples

## CLI Reporting

Run the `report!` macro example to see the output behavior of the different levels of verbosity.

The verbosity level argument is optional and will default to `quite` if omitted.

Use the following commands from the project root folder to run the examples

* __verbose__
```shell
cargo run --release --example cli-reporting --features="report,verbosity" -- verbose
```

* __terse__
```shell
cargo run --release --example cli-reporting --features="report,verbosity" -- terse
```

* __quite__
```shell
cargo run --release --example cli-reporting --features="report,verbosity"
```
```shell
cargo run --release --example cli-reporting --features="report,verbosity" -- quite
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

## CLI Eval

Run the `eval!` macro example to see the behavior of the different levels of verbosity.

The verbosity level argument is optional and will default to `quite` if omitted.

Use the following commands from the project root folder to run the examples

* __verbose__
```shell
cargo run --release --example cli-eval --features="debug,eval,report,verbosity" -- verbose
```

* __terse__
```shell
cargo run --release --example cli-eval --features="debug,eval,report,verbosity" -- terse
```

* __quite__
```shell
cargo run --release --example cli-eval --features="debug,eval,report,verbosity"
```
```shell
cargo run --release --example cli-eval --features="debug,eval,report,verbosity" -- quite
```

## CLI Release

Run the `release!` macro example to see the behavior of the different levels of verbosity.

The verbosity level argument is optional and will default to `quite` if omitted.

Use the following commands from the project root folder to run the examples

### Optimized

* __verbose__
```shell
cargo run --release --example cli-eval --features="debug,eval,report,verbosity" -- verbose
```

* __terse__
```shell
cargo run --release --example cli-eval --features="debug,eval,report,verbosity" -- terse
```

* __quite__
```shell
cargo run --release --example cli-eval --features="debug,eval,report,verbosity"
```
```shell
cargo run --release --example cli-eval --features="debug,eval,report,verbosity" -- quite
```

### Unoptimized

* __verbose__
```shell
cargo run --example cli-eval --features="debug,eval,report,verbosity" -- verbose
```

* __terse__
```shell
cargo run --example cli-eval --features="debug,eval,report,verbosity" -- terse
```

* __quite__
```shell
cargo run --example cli-eval --features="debug,eval,report,verbosity"
```
```shell
cargo run --example cli-eval --features="debug,eval,report,verbosity" -- quite
```
