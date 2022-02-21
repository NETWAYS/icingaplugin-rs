[![GitHub release (latest by date)](https://img.shields.io/github/v/release/NETWAYS/icingaplugin-rs?label=version)](https://github.com/NETWAYS/icingaplugin-rs/releases)
![GitHub](https://img.shields.io/github/license/NETWAYS/vspheredb-data-check?color=green)

# icingaplugin-rs

icingaplugin-rs is a collection of utilities useful for writing standardized check plugins for Icinga2 and the likes written in Rust. 
Right now, this library is a heavy work in progress and not published to [crates.io](https://crates.io). This may change in the future.

## Usage
Add the repository as dependency in your project's `Cargo.toml`:

```toml
[package]
name = "dummy_check_example"
authors = [ "Me <me@mail.com>"]
edition = 2018

[dependencies]
icingaplugin-rs = { git = "https://github.com/NETWAYS/icingaplugin-rs", tag = "v0.0.1"}
```

As mentioned, this is a heavy work in progress and no usage examples will be given right now as they might change at any time. For guidance, take a look at the `docstrings` provided
in the source code (e.g. [check.rs]("src/check.rs")).

## License 

Copyright© 2022 [NETWAYS GmbH](mailto:info@netways.de)

This check plugin is distributed under the GPL-2.0 or newer license shipped with this repository in the [LICENSE](LICENSE) file.
