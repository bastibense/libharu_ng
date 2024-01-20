use libharu_ng as lh;

fn main() {
    println!("Hello, world!");

    let doc = lh::document::PdfDocument::new();
    doc.save_to_file("test.pdf");
}
