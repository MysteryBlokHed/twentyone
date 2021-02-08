<h1 align="center">twentyone</h1>
<!-- Shields.io Badges -->
<p align="center">
    <a href="https://crates.io/crates/twentyone"><img src="https://img.shields.io/crates/v/twentyone"></a>
    <a href="https://docs.rs/twentyone/"><img src="https://docs.rs/twentyone/badge.svg"></a>
    <a href="#license"><img src="https://img.shields.io/github/license/MysteryBlokHed/twentyone"></a>
</p>
<!-- End of Badges -->
<p align="center">A blackjack engine for Rust.</p>

## Building

To build the project, run `cargo build` in the project's root directory.

To build project documentation, run `cargo doc` in the project's root directory.
Generated documentation will be available at `/target/doc/twentyone/index.html`.

## Including as a dependency

### Via crates.io

To add this project as a dependency via crates.io, add the following
to your `Cargo.toml` dependencies:

```toml
[dependencies]
twentyone = "0.1"
```

### Via git

To add this project as a dependency via the git repository,
add the following to your `Cargo.toml` dependencies:

```toml
[dependencies]
twentyone = { git = "https://github.com/MysteryBlokHed/twentyone" }
```

### Via a local build

After building this project, to add it as a dependency elsewhere,
add the following to your `Cargo.toml` dependencies:

```toml
[dependencies]
twentyone = { path = "/path/to/build/location" }
```

## Documentation

Documentation is available at <https://docs.rs/twentyone/>.

## License

This project is licensed under the Apache License, Version 2.0
([LICENSE](LICENSE) or <https://www.apache.org/licenses/LICENSE-2.0>).
