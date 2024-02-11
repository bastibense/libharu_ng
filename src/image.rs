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

//! The image struct and related functions.
//!

use crate::{haru_bindings as hb, prelude::HaruError};

/// The image object.
///
#[derive(Debug)]
pub struct PdfImage {
    /// The reference to the haru image.
    pub image_ref: hb::HPDF_Image,
}

impl PdfImage {
    /// get_width() returns the width of the image.
    ///
    /// Api: HPDF_Image_GetWidth
    ///
    pub fn get_width(&self) -> Result<u32, HaruError> {
        let result = unsafe { hb::HPDF_Image_GetWidth(self.image_ref) };
        match result {
            0 => Err(HaruError::from(0)),
            _ => Ok(result),
        }
    }

    /// get_height() returns the height of the image.
    ///
    /// Api: HPDF_Image_GetHeight
    ///
    pub fn get_height(&self) -> Result<u32, HaruError> {
        let result = unsafe { hb::HPDF_Image_GetHeight(self.image_ref) };
        match result {
            0 => Err(HaruError::from(0)),
            _ => Ok(result),
        }
    }
}
