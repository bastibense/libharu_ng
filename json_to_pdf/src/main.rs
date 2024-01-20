use libharu_ng::prelude::*;

fn main() {
    println!("Hello, world!");

    let doc = PdfDocument::new();

    let page = doc.add_page();
    page.set_width(402.0);

    doc.save_to_file("test.pdf");
}
