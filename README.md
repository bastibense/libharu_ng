# libharu_ng

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/bastibense/libharu_ng/master-build-test.yml)
![Crates.io Downloads](https://img.shields.io/crates/d/libharu_ng)
![GitHub Stars](https://img.shields.io/github/stars/bastibense/libharu_ng)

#### ⭐ Support this crate, be cool, [star this repository on GitHub](https://github.com/bastibense/libharu_ng)! :)

## What is it?

Using `libharu_ng`, you can easily create PDF documents from Rust code.

`libharu_ng` is a modern API wrapper for [libharu](http://libaru.org/) ([GitHub Repository](https://github.com/libharu/libharu_ng)). `libharu` is a C library for creating PDF files. This crate provides a modern and safe Rust API for generating PDFs without having to worry about the underlying C code.

So, until there are mature pure-Rust alternatives for generating PDFs, `libharu_ng` is a good choice for generating PDFs from Rust code without having to use a headless browser or a commercial solution.

## `libharu` Version

This crate uses the latest version of `libharu` (2.4.4) and will be updated to the latest version of `libharu` as soon as possible.

## Features

### Standard libharu Features

- Create PDF documents.
- Add pages to the document.
- Add text to pages.
  - Supporting text-wrapping into a given rectangle.
  - Use built-in fonts or load custom fonts (TTF).
- Add images to pages.
  - With support for JPEG and PNG images (including transparency).
- Use custom page sizes.
- Save and restore the graphics state.
- Set password protection for the document.
- Add shapes like lines, rectangles, circles, etc. to pages.
- Add annotations to pages.
- Add outlines to the document.
- Add metadata to the document.
- Compiles the libharu library from source, embedding it into your Rust project.

### Additional Features (provided by `libharu_ng`)

- Simple functions for CTM transformations.
  - Rotate, Translate, Scale, etc.

For more information about the features, please see the [libharu documentation](http://libharu.org).

## Requirements

- libz (required at runtime)
- libpng (required at runtime)
- cmake (required for building the C library)

### On Ubuntu/Debian

    $ apt-get install build-essential cmake libpng-dev libz3-dev

### On macOS (Homebrew)

    $ brew install cmake libpng zlib

## Usage

To use `libharu` in your Rust project, run the command line, in your Rust project directory:

```bash
$ cargo add libharu_ng
```

## Example

```rust
// Example is work in progress.

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

# Motivation

The main motivation behind `libharu_ng` is to provide a simple and modern API for generating PDFs from Rust code.

One of the requirements was fine-grained control over the content of the PDF document and minimal dependencies and size/performance overhead.

There are a number of alternatives for generating PDFs from Rust code, each with their own advantages and disadvantages:

This crate, as is, will try to compile the `libharu` library from source, embedding it into your Rust project. This means that you don't have to install any additional dependencies on your system, and you don't have to worry about the `libharu` version on your system. If you have problems with the embedded `libharu` version, please open an issue.

# Contributing

Contributions are welcome. Please open an issue before submitting a pull request.

# License

- This project is licensed under the MIT license.
- Haru is distributed under the ZLIB/LIBPNG License.
