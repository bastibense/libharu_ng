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

//! Rust bindings for libharu.
//!
//! libharu is a free, cross platform, open source library for generating PDF files.
//!
//! This crate provides a high level API for creating PDF files.
//!

mod haru_bindings;

pub mod document;
pub mod font;
pub mod haru_types;
pub mod image;
pub mod page;

/// The prelude module.
///
pub mod prelude {
    pub use crate::document::*;
    pub use crate::font::*;
    pub use crate::haru_types::*;
    pub use crate::image::*;
    pub use crate::page::*;
}

/// The error callback function type
pub type ErrorCallback =
    extern "C" fn(error_no: i32, detail_no: i32, user_data: *mut std::ffi::c_void);

pub struct HpdfBox {
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
    pub top: f32,
}

/// Conversion from HPDF_Box to HpdfBox
///
impl From<haru_bindings::HPDF_Box> for HpdfBox {
    fn from(hpdf_box: haru_bindings::HPDF_Box) -> Self {
        HpdfBox {
            left: hpdf_box.left,
            bottom: hpdf_box.bottom,
            right: hpdf_box.right,
            top: hpdf_box.top,
        }
    }
}

/// Returns the version of the libharu library.
/// The version is a string in the format "major.minor.patch".
///
pub fn libharu_version() -> String {
    let version = unsafe { haru_bindings::HPDF_GetVersion() };
    let version = unsafe { std::ffi::CStr::from_ptr(version) };
    version.to_str().unwrap().to_string()
}

/// Test which prints the version.
///
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_libharu_version() {
        let version = libharu_version();
        println!("libharu version: {}", version);
    }
}
