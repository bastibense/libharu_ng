//! PDF-Document related structs and functions.
//!
//! ## Implementation Status
//!
//! ### Fonts
//! - [x] HPDF_LoadType1FontFromFile()
//! - [x] HPDF_LoadTTFontFromFile()
//! - [x] HPDF_LoadTTFontFromFile2()
//! - [x] HPDF_UseCNSFonts()
//! - [x] HPDF_UseCNTFonts()
//! - [x] HPDF_UseJPFonts()
//! - [x] HPDF_UseKRFonts()
//!
//! ### Encodings
//! - [ ] HPDF_Encoder_GetType()
//! - [ ] HPDF_Encoder_GetByteType()
//! - [ ] HPDF_Encoder_GetUnicode()
//! - [ ] HPDF_Encoder_GetWritingMode()
//!

use crate::{
    font::PdfFont,
    haru_bindings as hb,
    haru_types::{CompressionMode, HaruError, PageLayout, PageMode},
    page::PdfPage,
};

/// The PDF document.
///
#[derive(Debug)]
pub struct PdfDocument {
    /// The reference to the haru document.
    pub doc: hb::HPDF_Doc,
}

impl Default for PdfDocument {
    fn default() -> Self {
        Self::new()
    }
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
    /// TODO: Should not cast int type?
    ///
    pub fn get_error(&self) -> HaruError {
        let error = unsafe { hb::HPDF_GetError(self.doc) };
        HaruError::from(error as u32)
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

    /// load_tt_font_from_file() loads a TrueType font file and returns a font object.
    ///
    /// API: HPDF_LoadTTFontFromFile
    ///
    pub fn load_tt_font_from_file(&self, filename: &str, embedding: bool) -> PdfFont {
        let filename = std::ffi::CString::new(filename).unwrap();
        let embedding = if embedding { 1 } else { 0 };
        let fontname =
            unsafe { hb::HPDF_LoadTTFontFromFile(self.doc, filename.as_ptr(), embedding) };
        let font = unsafe { hb::HPDF_GetFont(self.doc, fontname, core::ptr::null_mut()) };
        PdfFont { font_ref: font }
    }

    /// load_tt_font_from_file2() loads a TrueType font file and returns a font object.
    ///
    /// API: HPDF_LoadTTFontFromFile2
    ///
    pub fn load_tt_font_from_file2(&self, filename: &str, index: u32, embedding: bool) -> PdfFont {
        let filename = std::ffi::CString::new(filename).unwrap();
        let embedding = if embedding { 1 } else { 0 };
        let fontname =
            unsafe { hb::HPDF_LoadTTFontFromFile2(self.doc, filename.as_ptr(), index, embedding) };
        let font = unsafe { hb::HPDF_GetFont(self.doc, fontname, core::ptr::null_mut()) };
        PdfFont { font_ref: font }
    }

    /// HPDF_LoadType1FontFromFile() loads a Type1 font file and returns a font object.
    ///
    /// API: HPDF_LoadType1FontFromFile
    ///
    pub fn load_type1_font_from_file(&self, afm_filename: &str, pfm_filename: &str) -> PdfFont {
        let afm_filename = std::ffi::CString::new(afm_filename).unwrap();
        let pfm_filename = std::ffi::CString::new(pfm_filename).unwrap();
        let fontname = unsafe {
            hb::HPDF_LoadType1FontFromFile(self.doc, afm_filename.as_ptr(), pfm_filename.as_ptr())
        };
        let font = unsafe { hb::HPDF_GetFont(self.doc, fontname, core::ptr::null_mut()) };
        PdfFont { font_ref: font }
    }

    /// HPDF_UseCNSFonts() loads Chinese simplified fonts.
    ///
    /// API: HPDF_UseCNSFonts
    ///
    pub fn use_cns_fonts(&self) {
        unsafe { hb::HPDF_UseCNSFonts(self.doc) };
    }

    /// HPDF_UseCNTFonts() loads Chinese traditional fonts.
    ///
    /// API: HPDF_UseCNTFonts
    ///
    pub fn use_cnt_fonts(&self) {
        unsafe { hb::HPDF_UseCNTFonts(self.doc) };
    }

    /// HPDF_UseJPFonts() loads Japanese fonts.
    ///
    /// API: HPDF_UseJPFonts
    ///
    pub fn use_jp_fonts(&self) {
        unsafe { hb::HPDF_UseJPFonts(self.doc) };
    }

    /// HPDF_UseKRFonts() loads Korean fonts.
    ///
    /// API: HPDF_UseKRFonts
    ///
    pub fn use_kr_fonts(&self) {
        unsafe { hb::HPDF_UseKRFonts(self.doc) };
    }
}

impl Drop for PdfDocument {
    fn drop(&mut self) {
        unsafe { hb::HPDF_Free(self.doc) };
    }
}
