# libharu_ng

# ⚠️ EXPERIMENTAL WORK IN PROGRESS ⚠️

# ⭐ If you are interested in this project, please star this repository on GitHub to let us know! ⭐

A Rust API for [libharu](http://libaru.org/).

## Requirements

- System provided [libharu](http://libharu.org/) (tested with version 2.4).

## Usage

To use `libharu` in your Rust program add the following to your `Cargo.toml` file:

```toml
[dependencies]
libharu_ng = "2.4.0"
```

or

on the command line:

```bash
$ cargo add libharu_ng
```

# Contributing

Contributions are welcome. Please open an issue before submitting a pull request.

### To-Do List

- [ ] Figure out how to handle deployment of the C library (for systems without a package for it).
- [ ] Add more documentation.
- [ ] Better error handling.
- [ ] Add more examples.
- [ ] Add more tests.

# About

This is just a wrapper around the C library [libharu](http://libharu.org/).

# License

This project is licensed under the MIT license.
