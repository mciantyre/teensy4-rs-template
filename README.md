# teensy4-rs-template

[![Check Template][check-template-badge]][check-template-url]

[check-template-badge]: https://github.com/mciantyre/teensy4-rs/workflows/Check%20Template/badge.svg
[check-template-url]: https://github.com/mciantyre/teensy4-rs/actions?query=workflow%3A%22Check+Template%22

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