mod haru_bindings;

pub mod document;
pub mod haru_types;
pub mod page;

pub mod prelude {
    pub use crate::document::*;
    pub use crate::haru_types::*;
    pub use crate::page::*;
}

/// The error callback function type.
pub type ErrorCallback =
    extern "C" fn(error_no: i32, detail_no: i32, user_data: *mut std::ffi::c_void);

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
