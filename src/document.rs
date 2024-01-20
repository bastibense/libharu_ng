use crate::haru_bindings as hb;
use crate::page::PdfPage;

/// The PDF document.
///
pub struct PdfDocument {
    /// The reference to the haru document.
    pub doc: hb::HPDF_Doc,
}

impl PdfDocument {
    /// Create an instance of a document object and initialize it.
    ///
    pub fn new() -> Self {
        let doc = unsafe { hb::HPDF_New(None, core::ptr::null_mut()) };
        Self { doc }
    }

    /// Save the PDF document to a file.
    ///
    pub fn save_to_file(&self, filename: &str) {
        let filename = std::ffi::CString::new(filename).unwrap();
        unsafe { hb::HPDF_SaveToFile(self.doc, filename.as_ptr()) };
    }

    /// Returns the last error code of specified document object.
    ///
    pub fn get_error(&self) -> hb::HPDF_STATUS {
        unsafe { hb::HPDF_GetError(self.doc) }
    }

    /// Create a new page.
    ///
    pub fn add_page(&self) -> PdfPage {
        let page = unsafe { hb::HPDF_AddPage(self.doc) };
        PdfPage { page }
    }

    /// Set the title of the document.
    ///
    pub fn set_title(&self, title: &str) {
        let title = std::ffi::CString::new(title).unwrap();
        unsafe {
            hb::HPDF_SetInfoAttr(self.doc, hb::_HPDF_InfoType_HPDF_INFO_TITLE, title.as_ptr())
        };
    }
}

impl Drop for PdfDocument {
    fn drop(&mut self) {
        unsafe { hb::HPDF_Free(self.doc) };
    }
}
