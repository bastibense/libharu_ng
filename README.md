# libharu_ng

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/bastibense/libharu_ng/master-build-test.yml)

#### ⭐ Support us, be cool, [star this repository on GitHub](https://github.com/bastibense/libharu_ng)! :)

#### ⚠️ This is Work In Progress. The API might change at any time.

## What is it?

` libharu_ng` is a modern API wrapper for [libharu](http://libaru.org/) ([GitHub Repository](https://github.com/libharu/libharu)).

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

# Contributing

Contributions are welcome. Please open an issue before submitting a pull request.

### To-Do List

- [ ] Add more documentation.
- [ ] Add more examples.
- [ ] Add more tests.

# License

This project is licensed under the MIT license.
Haru is distributed under the ZLIB/LIBPNG License.
