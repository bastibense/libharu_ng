use libharu_ng::prelude::*;

/// Hello World example.
///
/// This example will create a new PDF file named `hello_world.pdf` in the current directory.
///

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
