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
    pub fn set_width(&self, width: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetWidth(self.page, width) };
        self
    }

    /// Set the height of the page.
    ///
    pub fn set_height(&self, height: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetHeight(self.page, height) };
        self
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

    /// Appends a rectangle to the current path.
    ///
    pub fn rectangle(&self, x: f32, y: f32, width: f32, height: f32) -> &Self {
        unsafe { hb::HPDF_Page_Rectangle(self.page, x, y, width, height) };
        self
    }

    /// Fills the current path using the nonzero winding number rule.
    ///
    pub fn fill(&self) -> &Self {
        unsafe { hb::HPDF_Page_Fill(self.page) };
        self
    }

    /// g_save saves the page's current graphics parameter to the stack.
    /// An application can invoke HPDF_Page_GSave() up to 28 (???) and can
    /// restore the saved parameter by invoking HPDF_Page_GRestore().
    ///
    /// The parameters that are saved by HPDF_Page_GSave() are:
    ///
    /// - Character Spacing
    /// - Clipping Path
    /// - Dash Mode
    /// - Filling Color
    /// - Flatness
    /// - Font
    /// - Font Size
    /// - Horizontal Scalling
    /// - Line Width
    /// - Line Cap Style
    /// - Line Join Style
    /// - Miter Limit
    /// - Rendering Mode
    /// - Stroking Color
    /// - Text Leading
    /// - Text Rise
    /// - Transformation Matrix
    /// - Word Spacing
    ///
    pub fn g_save(&self) -> &Self {
        unsafe { hb::HPDF_Page_GSave(self.page) };
        self
    }

    /// g_restore() restore the graphics state which is saved by HPDF_Page_GSave().
    ///
    pub fn g_restore(&self) -> &Self {
        unsafe { hb::HPDF_Page_GRestore(self.page) };
        self
    }

    /// appends a path from the current point to the specified point.
    ///
    pub fn line_to(&self, x: f32, y: f32) -> &Self {
        unsafe { hb::HPDF_Page_LineTo(self.page, x, y) };
        self
    }

    /// Changes the current text position, using the specified offset values.
    /// If the current text position is (x1, y1), the new text position
    /// will be (x1 + x, y1 + y).
    ///
    pub fn move_text_pos(&self, x: f32, y: f32) -> &Self {
        unsafe { hb::HPDF_Page_MoveTextPos(self.page, x, y) };
        self
    }

    /// Starts a new subpath and move the current point for drawing path,
    /// HPDF_Page_MoveTo() sets the start point for the path to the point (x, y).
    ///
    pub fn move_to(&self, x: f32, y: f32) -> &Self {
        unsafe { hb::HPDF_Page_MoveTo(self.page, x, y) };
        self
    }

    /// HPDF_Page_SetRGBFill() sets the filling color.
    ///
    pub fn set_rgb_fill(&self, r: f32, g: f32, b: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetRGBFill(self.page, r, g, b) };
        self
    }

    /// HPDF_Page_SetRGBStroke() sets the stroking color.
    ///
    pub fn set_rgb_stroke(&self, r: f32, g: f32, b: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetRGBStroke(self.page, r, g, b) };
        self
    }

    /// HPDF_Page_SetTextLeading() sets the text leading (line spacing) for text showing.
    ///
    pub fn set_text_leading(&self, value: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetTextLeading(self.page, value) };
        self
    }

    /// HPDF_Page_SetTextMatrix sets a transformation matrix for text to be drawn in using HPDF_Page_ShowText().
    ///
    pub fn set_text_matrix(&self, a: f32, b: f32, c: f32, d: f32, x: f32, y: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetTextMatrix(self.page, a, b, c, d, x, y) };
        self
    }

    /// HPDF_Page_SetTextRise() moves the text position in vertical direction
    /// by the amount of value. Useful for making subscripts or superscripts.
    ///
    pub fn set_text_rise(&self, value: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetTextRise(self.page, value) };
        self
    }

    /// HPDF_Page_SetWordSpace() sets the word spacing for text.
    ///
    pub fn set_word_space(&self, value: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetWordSpace(self.page, value) };
        self
    }

    /// HPDF_Page_ShowText() prints the text at the current position on the page.
    ///
    pub fn show_text(&self, text: &str) -> &Self {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { hb::HPDF_Page_ShowText(self.page, text.as_ptr()) };
        self
    }

    /// HPDF_Page_ShowTextNextLine() moves the current text position to the
    /// start of the next line, then prints the text at the current position on the page.
    ///
    pub fn show_text_next_line(&self, text: &str) -> &Self {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { hb::HPDF_Page_ShowTextNextLine(self.page, text.as_ptr()) };
        self
    }

    /// HPDF_Page_ShowTextNextLineEx() moves the current text position to the start of
    /// the next line; then sets word spacing and character spacing; finally prints
    /// the text at the current position on the page.
    ///
    pub fn show_text_next_line_ex(&self, word_space: f32, char_space: f32, text: &str) -> &Self {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            hb::HPDF_Page_ShowTextNextLineEx(self.page, word_space, char_space, text.as_ptr())
        };
        self
    }

    /// HPDF_Page_Stroke() paints the current path.
    ///
    pub fn stroke(&self) -> &Self {
        unsafe { hb::HPDF_Page_Stroke(self.page) };
        self
    }

    /// HPDF_Page_TextOut() prints the text on the specified position.
    ///
    pub fn text_out(&self, x: f32, y: f32, text: &str) -> &Self {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { hb::HPDF_Page_TextOut(self.page, x, y, text.as_ptr()) };
        self
    }
}
