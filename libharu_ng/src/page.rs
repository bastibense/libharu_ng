use crate::haru_bindings as hb;

/// Th PDF Page object.
///
pub struct PdfPage {
    /// The reference to the haru page.
    pub page: hb::HPDF_Page,
}

impl PdfPage {
    /// Set the width of the page.
    ///
    pub fn set_width(&self, width: f32) {
        unsafe { hb::HPDF_Page_SetWidth(self.page, width) };
    }

    /// Set the height of the page.
    ///
    pub fn set_height(&self, height: f32) {
        unsafe { hb::HPDF_Page_SetHeight(self.page, height) };
    }

    /// Get the width of the page.
    ///
    pub fn get_width(&self) -> f32 {
        unsafe { hb::HPDF_Page_GetWidth(self.page) }
    }

    /// Get the height of the page.
    ///   
    pub fn get_height(&self) -> f32 {
        unsafe { hb::HPDF_Page_GetHeight(self.page) }
    }
}
