// Copyright (c) 2023-2024 Bastian Bense
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Contact: Bastian Bense, bb@neosw.de

//! The Font module.
//!
//! For more information about using fonts in Haru, see the []
//!
//! ## Implementation status:
//!
//! - [x] HPDF_Font_GetFontName()
//! - [x] HPDF_Font_GetEncodingName()
//! - [x] HPDF_Font_GetUnicodeWidth()
//! - [x] HPDF_Font_GetBBox()
//! - [x] HPDF_Font_GetAscent()
//! - [x] HPDF_Font_GetDescent()
//! - [x] HPDF_Font_GetXHeight()
//! - [x] HPDF_Font_GetCapHeight()
//! - [x] HPDF_Font_TextWidth()
//! - [ ] HPDF_Font_MeasureText()
//!

use crate::{haru_bindings as hb, HpdfBox};

/// The font object.
///
#[derive(Debug, Copy, Clone)]
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

    /// HPDF_Font_GetBBox() gets the bounding box of the font.
    ///
    /// When HPDF_Font_GetBox() succeed, it returns the HPDF_Box struct specifying the font bounding box. Otherwise, it returns a HPDF_Box struct of {0, 0, 0, 0}.
    ///
    /// API: HPDF_Box HPDF_Font_GetBBox

    pub fn get_b_box(&self) -> HpdfBox {
        unsafe { HpdfBox::from(hb::HPDF_Font_GetBBox(self.font_ref)) }
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

    /// HPDF_Font_TextWidth() gets total width of the text, number of charactors and number of the words.
    ///
    /// When HPDF_Font_TextWidth() succeed, it returns a HPDF_TextWidth struct including calculation result. Otherwise, it returns a HPDF_TextWidth struct whose attributes are all ZERO.
    ///
    pub fn text_width(&self, text: &str) -> hb::HPDF_TextWidth {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            hb::HPDF_Font_TextWidth(
                self.font_ref,
                text.as_ptr() as *const u8,
                text.as_bytes().len() as u32,
            )
        }
    }
}
