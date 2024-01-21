// Copyright (c) 2022 Bastian Bense
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

/// The error callback function type.
pub type ErrorCallback =
    extern "C" fn(error_no: i32, detail_no: i32, user_data: *mut std::ffi::c_void);

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn it_works() -> Result<(), HaruError> {
        let doc = PdfDocument::new();
        doc.add_page()?;

        doc.save_to_file("hello_world.pdf")
            .expect("Save to file failed");

        Ok(())
    }
}
