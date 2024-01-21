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
    pub fn set_page_mode(&self, mode: PageMode) -> Result<&Self, HaruError> {
        let result = unsafe { hb::HPDF_SetPageMode(self.doc, mode.to_hpdf_mode()) };
        match result {
            0 => Ok(self),
            _ => Err(HaruError::from(result as u32)),
        }
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
    pub fn set_page_layout(&self, layout: PageLayout) -> Result<&Self, HaruError> {
        let result = unsafe { hb::HPDF_SetPageLayout(self.doc, layout.to_hpdf_layout()) };
        match result {
            0 => Ok(self),
            _ => Err(HaruError::from(result as u32)),
        }
    }

    /// HPDF_GetPageLayout() returns the current setting for page layout.
    ///
    pub fn get_page_layout(&self) -> PageLayout {
        let layout = unsafe { hb::HPDF_GetPageLayout(self.doc) };
        PageLayout::from_hpdf_layout(layout)
    }

    /// Save the PDF document to a file.
    ///
    pub fn save_to_file(&self, filename: &str) -> Result<&Self, HaruError> {
        let filename = std::ffi::CString::new(filename).unwrap();
        let result = unsafe { hb::HPDF_SaveToFile(self.doc, filename.as_ptr()) };
        match result {
            0 => Ok(self),
            _ => Err(HaruError::from(result as u32)),
        }
    }

    /// Returns the last error code of specified document object.
    ///
    /// API: HPDF_GetError
    ///
    pub fn get_error(&self) -> HaruError {
        // TODO: Should not cast int type?
        let error = unsafe { hb::HPDF_GetError(self.doc) };
        HaruError::from(error as u32)
    }

    /// Create a new page.
    ///
    /// API: HPDF_AddPage
    ///
    pub fn add_page(&self) -> PdfPage {
        // TODO: Error handling.
        let page: *mut hb::_HPDF_Dict_Rec = unsafe { hb::HPDF_AddPage(self.doc) };
        PdfPage { page }
    }

    /// insert_page() creates a new page and inserts it just before the specified page.
    ///
    /// API: HPDF_InsertPage
    ///
    pub fn insert_page(&self, page: PdfPage) -> PdfPage {
        // TODO: Error handling.
        let page: *mut hb::_HPDF_Dict_Rec = unsafe { hb::HPDF_InsertPage(self.doc, page.page) };
        PdfPage { page }
    }

    /// Set the title of the document.
    ///
    pub fn set_title(&self, title: &str) -> Result<&Self, HaruError> {
        let title = std::ffi::CString::new(title).unwrap();
        let result = unsafe {
            hb::HPDF_SetInfoAttr(self.doc, hb::_HPDF_InfoType_HPDF_INFO_TITLE, title.as_ptr())
        };
        match result {
            0 => Ok(self),
            _ => Err(HaruError::from(result as u32)),
        }
    }

    /// HPDF_SetPassword() sets a password for the document. If the password is set, document contents are encrypted.
    ///
    pub fn set_password(&self, owner_passwd: &str, user_passwd: &str) -> Result<&Self, HaruError> {
        let owner_passwd = std::ffi::CString::new(owner_passwd).unwrap();
        let user_passwd = std::ffi::CString::new(user_passwd).unwrap();
        let result =
            unsafe { hb::HPDF_SetPassword(self.doc, owner_passwd.as_ptr(), user_passwd.as_ptr()) };
        match result {
            0 => Ok(self),
            _ => Err(HaruError::from(result as u32)),
        }
    }

    /// HPDF_SetCompressionMode() set the mode of compression.
    ///
    pub fn set_compression_mode(&self, mode: CompressionMode) -> Result<&Self, HaruError> {
        let result = unsafe { hb::HPDF_SetCompressionMode(self.doc, mode.to_hpdf_compression()) };
        match result {
            0 => Ok(self),
            _ => Err(HaruError::from(result as u32)),
        }
    }

    /// load_tt_font_from_file() loads a TrueType font file and returns a font object.
    ///
    /// API: HPDF_LoadTTFontFromFile
    ///
    pub fn load_tt_font_from_file(&self, filename: &str, embedding: bool) -> PdfFont {
        // TODO: Error handling.
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
        // TODO: Error handling.
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
        // TODO: Error handling.
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
    pub fn use_cns_fonts(&self) -> Result<&Self, HaruError> {
        let result = unsafe { hb::HPDF_UseCNSFonts(self.doc) };
        match result {
            0 => Ok(self),
            _ => Err(HaruError::from(result as u32)),
        }
    }

    /// HPDF_UseCNTFonts() loads Chinese traditional fonts.
    ///
    /// API: HPDF_UseCNTFonts
    ///
    pub fn use_cnt_fonts(&self) -> Result<&Self, HaruError> {
        let result = unsafe { hb::HPDF_UseCNTFonts(self.doc) };
        match result {
            0 => Ok(self),
            _ => Err(HaruError::from(result as u32)),
        }
    }

    /// HPDF_UseJPFonts() loads Japanese fonts.
    ///
    /// API: HPDF_UseJPFonts
    ///
    pub fn use_jp_fonts(&self) -> Result<&Self, HaruError> {
        let result = unsafe { hb::HPDF_UseJPFonts(self.doc) };
        match result {
            0 => Ok(self),
            _ => Err(HaruError::from(result as u32)),
        }
    }

    /// HPDF_UseKRFonts() loads Korean fonts.
    ///
    /// API: HPDF_UseKRFonts
    ///
    pub fn use_kr_fonts(&self) -> Result<&Self, HaruError> {
        let result = unsafe { hb::HPDF_UseKRFonts(self.doc) };
        match result {
            0 => Ok(self),
            _ => Err(HaruError::from(result as u32)),
        }
    }
}

impl Drop for PdfDocument {
    fn drop(&mut self) {
        unsafe { hb::HPDF_Free(self.doc) };
    }
}
