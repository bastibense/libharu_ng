//! The Font module.
//!
//! For more information about using fonts in Haru, see the []
//!
//! ## Implementation status:
//!
//! - [x] HPDF_Font_GetFontName()
//! - [x] HPDF_Font_GetEncodingName()
//! - [x] HPDF_Font_GetUnicodeWidth()
//! - [ ] HPDF_Font_GetBBox()
//! - [x] HPDF_Font_GetAscent()
//! - [x] HPDF_Font_GetDescent()
//! - [x] HPDF_Font_GetXHeight()
//! - [x] HPDF_Font_GetCapHeight()
//! - [ ] HPDF_Font_TextWidth()
//! - [ ] HPDF_Font_MeasureText()
//!

use crate::haru_bindings as hb;

/// The font object.
///
pub struct PdfFont {
    /// The reference to the haru font.
    pub font_ref: hb::HPDF_Font,
}

impl PdfFont {
    /// HPDF_Font_GetFontName() gets the name of the font.
    ///
    pub fn get_font_name(&self) -> String {
        let name = unsafe { hb::HPDF_Font_GetFontName(self.font_ref) };
        let name = unsafe { std::ffi::CStr::from_ptr(name) };
        name.to_str().unwrap().to_string()
    }

    /// HPDF_Font_GetEncodingName() gets the encoding name of the font.
    ///
    pub fn get_encoding_name(&self) -> String {
        let name = unsafe { hb::HPDF_Font_GetEncodingName(self.font_ref) };
        let name = unsafe { std::ffi::CStr::from_ptr(name) };
        name.to_str().unwrap().to_string()
    }

    /// HPDF_Font_GetUnicodeWidth() gets the width of a Unicode character
    /// in a specific font. Actual width of the character on the page
    /// can be calculated as follows:
    ///
    /// ```c
    /// char_width = HPDF_Font_GetUnicodeWidth (font, UNICODE);
    /// float actual_width = char_width * FONT_SIZE / 1000;
    /// ````
    ///
    /// FIXME: Replace ex. C code with Rust code.
    ///
    pub fn get_unicode_width(&self, code: u16) -> i32 {
        unsafe { hb::HPDF_Font_GetUnicodeWidth(self.font_ref, code) }
    }

    /// HPDF_Font_GetAscent() gets the vertical ascent of the font.
    ///
    pub fn get_ascent(&self) -> i32 {
        unsafe { hb::HPDF_Font_GetAscent(self.font_ref) }
    }

    /// HPDF_Font_GetDescent() gets the vertical descent of the font.
    ///
    pub fn get_descent(&self) -> i32 {
        unsafe { hb::HPDF_Font_GetDescent(self.font_ref) }
    }

    /// HPDF_Font_GetXHeight() gets the distance from the baseline of lowercase letters.
    ///
    pub fn get_x_height(&self) -> u32 {
        unsafe { hb::HPDF_Font_GetXHeight(self.font_ref) }
    }

    /// HPDF_Font_GetCapHeight() gets the distance from the baseline of uppercase letters.
    ///
    pub fn get_cap_height(&self) -> u32 {
        unsafe { hb::HPDF_Font_GetCapHeight(self.font_ref) }
    }
}
