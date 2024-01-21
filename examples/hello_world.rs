use libharu_ng;

/// Hello World example.
///
/// This example will create a new PDF file named `hello_world.pdf` in the current directory.
///

fn main() {
    let doc = libharu_ng::document::PdfDocument::new();
    let page = doc.add_page();
    page.show_text("Hello World!");
    doc.save_to_file("hello_world.pdf");
    println!("hello_world.pdf has been created!");
}
