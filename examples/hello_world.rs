use libharu_ng::{self, document::PdfDocument};

/// Hello World example.
///
/// This example will create a new PDF file named `hello_world.pdf` in the current directory.
///

fn main() {
    let doc = PdfDocument::new();

    let page = doc.add_page();

    page.begin_text().expect("Begin text failed");

    page.move_text_pos(220.0, 20.0)
        .expect("Move text pos failed");

    let fnt = doc.get_font("Helvetica", None);
    page.set_font_and_size(fnt, 24.0)
        .expect("Set font and size failed");

    page.show_text("Hello World").expect("Show text failed");

    page.end_text().expect("End text failed");

    doc.save_to_file("./test.pdf").expect("Save to file failed");
}
