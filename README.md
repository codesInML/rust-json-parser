## A JSON Parser built with Rust

It takes a path to a JSON file as an argument, parses it and exits with non zero code if an invalid token was encounter.

Simply clone the repo and run

```bash
cargo run -- ./tests/step1/valid.json
```

or using the prebuilt executable
```bash
./target/release/json_parser ./tests/step1/valid.json
```