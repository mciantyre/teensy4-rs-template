# teensy4-rs-template

![Check Template](https://github.com/mciantyre/teensy4-rs/workflows/Check%20Template/badge.svg)

A [`cargo-generate`](https://crates.io/crates/cargo-generate) template for
Teensy 4 projects.

```
cargo install cargo-generate
cargo generate --git https://github.com/mciantyre/teensy4-rs-template --name hello-world
cd hello-world
cargo objcopy --release -- -O ihex hello-world.hex
```

See the [teensy4-rs
README](https://github.com/mciantyre/teensy4-rs/blob/master/README.md) for build requirements.