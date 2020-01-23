# teensy4-rs-template

A [`cargo-generate`](https://crates.io/crates/cargo-generate) template for
Teensy 4 projects.

```
cargo install cargo-generate
cargo generate --git https://github.com/mciantyre/teensy4-rs-template --name
hello-world
```

See the [teensy4-rs
README](https://github.com/mciantyre/teensy4-rs/blob/master/README.md) for build requirements.

Use the `hardware-test.sh` script to build and prepare a release build. If the
`teensy_cli_loader` is on your `$PATH`, we'll use it to load the binary on your
Teensy 4. The example should turn on your Teensy 4's LED.
