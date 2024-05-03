# Contributing

## Setup

Ensure you have make installed. Run `command -v make`. If there is no
stdout, search for ways to install `make` on your system.

```sh
make install # installs rust/cargo and cargo crate deps
```

## Building

To generate a release binary to `./target/release/tinted-builder-rust`, run:

```sh
make build
```

## Testing

### CLI

```sh
make cli_test
```

### Library

```sh
make lib_test
```
