mod haru_bindings;

pub mod document;
pub mod page;

pub mod prelude {
    pub use crate::document::PdfDocument;
    pub use crate::page::PdfPage;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn it_works() {
        let doc = PdfDocument::new();
        let page = doc.add_page();
        page.show_text("Hello World!");
        doc.save_to_file("hello_world.pdf");
    }
}
