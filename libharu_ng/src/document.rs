use crate::haru_bindings as hb;

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
}

impl Drop for PdfDocument {
    fn drop(&mut self) {
        unsafe { hb::HPDF_Free(self.doc) };
    }
}
