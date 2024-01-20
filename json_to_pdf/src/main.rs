use libharu_ng::prelude::*;

fn main() {
    println!("Hello, world!");

    let doc = PdfDocument::new();

    let page = doc.add_page();

    page.set_width(402.0)
        .g_save()
        .rectangle(10.0, 10.0, 100.0, 100.0)
        .fill()
        .g_restore();

    doc.save_to_file("test.pdf");
}
