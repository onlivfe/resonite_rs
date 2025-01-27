# Resonite API in rust

<img align="right" width="255" height="170" src="https://github.com/onlivfe/resonite_rs/raw/main/logo.png"/>

[![License](https://img.shields.io/crates/l/resonite.svg)](https://github.com/onlivfe/resonite_rs/src/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/resonite.svg)](https://crates.io/crates/resonite)
[![Docs](https://docs.rs/resonite/badge.svg)](https://docs.rs/crate/resonite/)

Rust models of [Resonite's](https://resonite.com) API.

Any official documentation of Resonite' API is lacking, and the API is still changing too.
So this crate can't guarantee correctness.

This crate provides an example API client with the optional `http_client` & `signalr_client` features.

For programming style, beyond the clippy lints & rustfmt's automatic formatting:
[Doc comments should come before attributes](https://github.com/rust-lang/rust/tree/HEAD/src/doc/style-guide/src#doc-comments).
I'm hoping that [someday rustfmt will gain the ability to automate that](https://github.com/rust-lang/rustfmt/issues/3744).

## Testing

The integration tests will contact the live API.
That's why they are ignored by default.

Some of them also will require authentication.

Sadly not all the things can even be reliably tested without creating a mock API.
Which in turn defeats the purpose of the tests in the first place.

### Generating auth token

To dogfed the API crate, the auth getting is implemented with a simple rust script using this crate itself.
You can run the binary with:

```sh
cargo run --bin auth-helper --all-features
```

### Running ignored tests

Make sure that you've got:

- an internet connection
- a valid authentication file

Then just run the tests;

```sh
# A specific test with output logging
cargo test --all-features get_user -- --exact --ignored --nocapture
# All tests
cargo test --all-features -- --ignored
```

## License

Note that the license is `MPL-2.0` instead of the more common `MIT OR Apache-2.0`.
A license change however can be negotiated if the Resonite team wants to use this crate or adopt this crate into a more official one with a different license.
