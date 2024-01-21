//!  PDF-Document related structs and functions.
//!

use crate::haru_bindings as hb;
use crate::haru_types::{CompressionMode, PageLayout, PageMode};
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

    /// HPDF_SetPageMode() sets how the document should be displayed.
    ///
    pub fn set_page_mode(&self, mode: PageMode) {
        unsafe { hb::HPDF_SetPageMode(self.doc, mode.to_hpdf_mode()) };
    }

    /// HPDF_GetPageLayout() returns the current setting for page mode.
    ///
    pub fn get_page_mode(&self) -> PageMode {
        let mode = unsafe { hb::HPDF_GetPageMode(self.doc) };
        PageMode::from_hpdf_mode(mode)
    }

    /// HPDF_SetPageLayout() sets how the page should be displayed. If this attribute
    /// is not set, the setting of the viewer application is used.
    ///
    pub fn set_page_layout(&self, layout: PageLayout) {
        unsafe { hb::HPDF_SetPageLayout(self.doc, layout.to_hpdf_layout()) };
    }

    /// HPDF_GetPageLayout() returns the current setting for page layout.
    ///
    pub fn get_page_layout(&self) -> PageLayout {
        let layout = unsafe { hb::HPDF_GetPageLayout(self.doc) };
        PageLayout::from_hpdf_layout(layout)
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
        let page: *mut hb::_HPDF_Dict_Rec = unsafe { hb::HPDF_AddPage(self.doc) };
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

    /// HPDF_SetPassword() sets a password for the document. If the password is set, document contents are encrypted.
    ///
    pub fn set_password(&self, owner_passwd: &str, user_passwd: &str) {
        let owner_passwd = std::ffi::CString::new(owner_passwd).unwrap();
        let user_passwd = std::ffi::CString::new(user_passwd).unwrap();
        unsafe { hb::HPDF_SetPassword(self.doc, owner_passwd.as_ptr(), user_passwd.as_ptr()) };
    }

    /// HPDF_SetCompressionMode() set the mode of compression.
    ///
    pub fn set_compression_mode(&self, mode: CompressionMode) {
        unsafe { hb::HPDF_SetCompressionMode(self.doc, mode.to_hpdf_compression()) };
    }
}

impl Drop for PdfDocument {
    fn drop(&mut self) {
        unsafe { hb::HPDF_Free(self.doc) };
    }
}
