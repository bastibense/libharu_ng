//! The image struct and related functions.
//!

use crate::haru_bindings as hb;

/// The image object.
///
#[derive(Debug)]
pub struct PdfImage {
    /// The reference to the haru image.
    pub image_ref: hb::HPDF_Image,
}
