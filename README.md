# libharu_ng

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/bastibense/libharu_ng/master-build-test.yml)

## ⚠️ EXPERIMENTAL WORK IN PROGRESS ⚠️

### ⭐ If you are interested in this project, [please star this repository on GitHub](https://github.com/bastibense/libharu_ng) to let us know! ⭐

A Rust API wrapper for [libharu](http://libaru.org/) ([GitHub Repository](https://github.com/libharu/libharu))

## Requirements

- Installed [libharu](http://libharu.org/) (tested with version 2.4).
  - See included Dockerfile for reference.

## Usage

To use `libharu` in your Rust project, run the command line, in your Rust project directory:

```bash
$ cargo add libharu_ng
```

## Example

```rust
use libharu_ng::prelude::*;

fn main() -> Result<(), HaruError> {
    let doc = PdfDocument::new();
    let fnt = doc.get_font("Helvetica", None)?;

    doc.add_page()?
        .begin_text()?
        .move_text_pos(220.0, 20.0)?
        .set_font_and_size(fnt, 24.0)?
        .show_text("Hello World")?
        .end_text()?;

    doc.save_to_file("./test.pdf")?;

    Ok(())
}
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
