# libharu_ng

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/bastibense/libharu_ng/master-build-test.yml)
![GitHub Stars](https://img.shields.io/github/stars/bastibense/libharu_ng)
![Crates.io Downloads](https://img.shields.io/crates/d/libharu_ng)

#### ⭐ Support us, be cool, [star this repository on GitHub](https://github.com/bastibense/libharu_ng)! :)

#### ⚠️ This is Work In Progress. The API might change at any time.

## What is it?

`libharu_ng` is a modern API wrapper for [libharu](http://libaru.org/) ([GitHub Repository](https://github.com/libharu/libharu)).

Using `libharu_ng`, you can easily create PDF documents from Rust code.

## Features

`libharu_ng` supports most features of `libharu`:

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

For more information about the features, please see the [libharu documentation](http://libharu.org).

## Requirements

- libz
- libpng
- cmake

### On Alpine Linux

    $ apk add --no-cache build-base libpng-dev zlib-dev cmake

### On Ubuntu/Debian

    $ apt-get install build-essential cmake libpng-dev libz3-dev

### On macOS (Homebrew)

    $ brew install cmake libpng zlib

### On Windows

    (Author doesn't have a Windows machine, some help would be appreciated.)

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

# Motivation & Alternatives

The main motivation behind `libharu_ng` is to provide a simple and modern API for generating PDFs from Rust code.

One of the requirements was fine-grained control over the content of the PDF document and minimal dependencies and size/performance overhead.

There are a number of alternatives for generating PDFs from Rust code, each with their own advantages and disadvantages:

- **lopdf** - [Crate](https://crates.io/crates/lopdf) - A pure Rust library for very low-level PDF manipulation.
  - **printpdf** - [Crate](https://crates.io/crates/printpdf) - A "higher" level API for generating PDFs, based on `lopdf`.
- **wkhtmltopdf** - [Crate](https://crates.io/crates/wkhtmltopdf) - A wrapper for the `wkhtmltopdf` command line tool - which is deprecated.
- Using a headless browser like Chromium. This will require a _lot_ of RAM - possibly more than your server has available, especially if you want to generate multiple PDFs in parallel.
- ...

This is not a complete list, but it should give you an idea of the alternatives. Of course there are commercial solutions as well, but for many use cases, they are not an option, overkill, require extensive integration or are too expensive.

This crate, as is, will try to compile the `libharu` library from source, embedding it into your Rust project. This means that you don't have to install any additional dependencies on your system, and you don't have to worry about the `libharu` version on your system. If you have problems with the embedded `libharu` version, please open an issue.

# Contributing

Contributions are welcome. Please open an issue before submitting a pull request.

### To-Do List

- [ ] Add more documentation.
- [ ] Add more examples.
- [ ] Add more tests.

# License

This project is licensed under the MIT license.
Haru is distributed under the ZLIB/LIBPNG License.
