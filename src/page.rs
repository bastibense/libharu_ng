use crate::haru_bindings as hb;

/// Th PDF Page object.
///
pub struct PdfPage {
    /// The reference to the haru page.
    pub page: hb::HPDF_Page,
}

impl PdfPage {
    /// HPDF_Page_Arc() appends a circle arc to the current path. Angles
    /// are given in degrees, with 0 degrees being vertical, upward, from
    /// the (x,y) position.
    ///
    pub fn arc(&self, x: f32, y: f32, radius: f32, ang1: f32, ang2: f32) -> &Self {
        unsafe { hb::HPDF_Page_Arc(self.page, x, y, radius, ang1, ang2) };
        self
    }

    /// HPDF_Page_BeginText() begins a text object and sets the text position to (0, 0).
    ///
    pub fn begin_text(&self) -> &Self {
        unsafe { hb::HPDF_Page_BeginText(self.page) };
        self
    }

    /// HPDF_Page_Circle() appends a circle to the current path.
    ///
    pub fn circle(&self, x: f32, y: f32, radius: f32) -> &Self {
        unsafe { hb::HPDF_Page_Circle(self.page, x, y, radius) };
        self
    }

    /// HPDF_Page_Clip() modifies the current clipping path by intersecting it with
    /// the current path using the nonzero winding number rule. The clipping path
    /// is only modified after the succeeding painting operator. To avoid painting
    /// the current path, use the function HPDF_Page_EndPath(). Following painting
    /// operations will only affect the regions of the page contained by the
    /// clipping path.
    ///
    /// Initially, the clipping path includes the entire page. There is no way to
    /// enlarge the current clipping path, or to replace the clipping path with a
    /// new one. The functions HPDF_Page_GSave() and HPDF_Page_GRestore() may be
    /// used to save and restore the current graphics state, including the clipping
    /// path.
    ///
    pub fn clip(&self) -> &Self {
        unsafe { hb::HPDF_Page_Clip(self.page) };
        self
    }

    /// HPDF_Page_ClosePath() appends a straight line from the current point to the
    /// start point of sub path. The current point is moved to the start point of
    /// sub path.
    ///
    pub fn close_path(&self) -> &Self {
        unsafe { hb::HPDF_Page_ClosePath(self.page) };
        self
    }

    /// HPDF_Page_ClosePathStroke() closes the current path. Then, it paints the path.
    ///
    pub fn close_path_stroke(&self) -> &Self {
        unsafe { hb::HPDF_Page_ClosePathStroke(self.page) };
        self
    }

    /// HPDF_Page_ClosePathEofillStroke() closes the current path, fills the current
    /// path using the even-odd rule, then paints the path.
    ///
    pub fn close_path_eofill_stroke(&self) -> &Self {
        unsafe { hb::HPDF_Page_ClosePathEofillStroke(self.page) };
        self
    }

    /// HPDF_Page_ClosePathFillStroke() closes the current path, fills the current
    /// path using the nonzero winding number rule, then paints the path.
    ///
    pub fn close_path_fill_stroke(&self) -> &Self {
        unsafe { hb::HPDF_Page_ClosePathFillStroke(self.page) };
        self
    }

    /// HPDF_Page_Concat() concatenates the page's current transformation matrix and specified matrix.
    ///
    pub fn concat(&self, a: f32, b: f32, c: f32, d: f32, x: f32, y: f32) -> &Self {
        unsafe { hb::HPDF_Page_Concat(self.page, a, b, c, d, x, y) };
        self
    }

    /// HPDF_Page_CurveTo() appends a Bézier curve to the current path using the control points (x1, y1)
    /// and (x2, y2) and (x3, y3), then sets the current point to (x3, y3).
    ///
    pub fn curve_to(&self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> &Self {
        unsafe { hb::HPDF_Page_CurveTo(self.page, x1, y1, x2, y2, x3, y3) };
        self
    }

    /// HPDF_Page_CurveTo2() appends a Bézier curve to the current path using the current point and
    /// (x2, y2) and (x3, y3) as control points. Then, the current point is set to (x3, y3).
    ///
    pub fn curve_to2(&self, x2: f32, y2: f32, x3: f32, y3: f32) -> &Self {
        unsafe { hb::HPDF_Page_CurveTo2(self.page, x2, y2, x3, y3) };
        self
    }

    /// HPDF_Page_CurveTo3() appends a Bézier curve to the current path using two spesified points.
    /// The point (x1, y1) and the point (x3, y3) are used as the control points for a Bézier curve
    /// and current point is moved to the point (x3, y3)
    ///
    pub fn curve_to3(&self, x1: f32, y1: f32, x3: f32, y3: f32) -> &Self {
        unsafe { hb::HPDF_Page_CurveTo3(self.page, x1, y1, x3, y3) };
        self
    }

    /// HPDF_Page_Ellipse() appends an ellipse to the current path.
    ///
    pub fn ellipse(&self, x: f32, y: f32, xray: f32, yray: f32) -> &Self {
        unsafe { hb::HPDF_Page_Ellipse(self.page, x, y, xray, yray) };
        self
    }

    /// HPDF_Page_EndPath() ends the path object without filling or painting.
    ///
    pub fn end_path(&self) -> &Self {
        unsafe { hb::HPDF_Page_EndPath(self.page) };
        self
    }

    /// HPDF_Page_EndText() ends a text object.
    ///
    pub fn end_text(&self) -> &Self {
        unsafe { hb::HPDF_Page_EndText(self.page) };
        self
    }

    /// HPDF_Page_Clip() modifies the current clipping path by intersecting it
    /// with the current path using the even-odd rule. The clipping path is only
    /// modified after the succeeding painting operator. To avoid painting the
    /// current path, use the function HPDF_Page_EndPath().
    ///
    /// Following painting operations will only affect the regions of the page
    /// contained by the clipping path. Initially, the clipping path includes
    /// the entire page. There is no way to enlarge the current clipping path,
    ///  or to replace the clipping path with a new one. The functions
    /// HPDF_Page_GSave() and HPDF_Page_GRestore() may be used to save and
    /// restore the current graphics state, including the clipping path.
    ///
    pub fn eo_clip(&self) -> &Self {
        unsafe { hb::HPDF_Page_Eoclip(self.page) };
        self
    }

    /// HPDF_Page_Eofill() fills the current path using the even-odd rule.
    ///
    pub fn eo_fill(&self) -> &Self {
        unsafe { hb::HPDF_Page_Eofill(self.page) };
        self
    }

    /// HPDF_Page_EofillStroke() fills the current path using the even-odd rule, then paints the path.
    ///
    pub fn eo_fill_stroke(&self) -> &Self {
        unsafe { hb::HPDF_Page_EofillStroke(self.page) };
        self
    }

    /// Draws the XObject using the current graphics context. This is used by HPDF_Page_DrawImage()
    /// to draw the HPDF_Image by first calling HPDF_Page_GSave() and HPDF_Page_Concat() and then
    /// calling HPDF_Page_GRestore() after HPDF_Page_ExecuteXObject(). It could be used manually
    /// to rotate an image.
    ///
    pub fn execute_xobject(&self, obj: hb::HPDF_XObject) -> &Self {
        unsafe { hb::HPDF_Page_ExecuteXObject(self.page, obj) };
        self
    }

    /// HPDF_Page_Fill() fills the current path using the nonzero winding number rule.
    ///
    pub fn fill(&self) -> &Self {
        unsafe { hb::HPDF_Page_Fill(self.page) };
        self
    }

    /// HPDF_Page_FillStroke() fills the current path using the nonzero winding number rule, then paints the path.
    ///
    pub fn fill_stroke(&self) -> &Self {
        unsafe { hb::HPDF_Page_FillStroke(self.page) };
        self
    }

    /// HPDF_Page_GRestore() restore the graphics state which is saved by HPDF_Page_GSave().
    ///
    pub fn g_restore(&self) -> &Self {
        unsafe { hb::HPDF_Page_GRestore(self.page) };
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

    /// appends a path from the current point to the specified point.
    ///
    pub fn line_to(&self, x: f32, y: f32) -> &Self {
        unsafe { hb::HPDF_Page_LineTo(self.page, x, y) };
        self
    }

    /// HPDF_Page_SetCharSpace() sets the character spacing for text.
    ///
    pub fn set_char_space(&self, value: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetCharSpace(self.page, value) };
        self
    }

    /// HPDF_Page_SetCMYKFill() sets the filling color.
    ///
    pub fn set_cmyk_fill(&self, c: f32, m: f32, y: f32, k: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetCMYKFill(self.page, c, m, y, k) };
        self
    }

    /// HPDF_Page_SetCMYKStroke() sets the stroking color.
    ///
    pub fn set_cmyk_stroke(&self, c: f32, m: f32, y: f32, k: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetCMYKStroke(self.page, c, m, y, k) };
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

    /// HPDF_Page_MoveTextPos2() changes the current text position, using the specified offset values. If the current text position is (x1, y1), the new text position will be (x1 + x, y1 + y).
    ///
    pub fn move_text_pos2(&self, x: f32, y: f32) -> &Self {
        unsafe { hb::HPDF_Page_MoveTextPos2(self.page, x, y) };
        self
    }

    /// HPDF_Page_MoveTo() starts a new subpath and move the current point for drawing path, HPDF_Page_MoveTo() sets the start point for the path to the point (x, y).
    ///
    pub fn move_to(&self, x: f32, y: f32) -> &Self {
        unsafe { hb::HPDF_Page_MoveTo(self.page, x, y) };
        self
    }

    /// HPDF_Page_MoveToNextLine() moves current position for the text showing depending on current text showing point and text leading. The new position is calculated with current text transition matrix.
    ///
    pub fn move_to_next_line(&self) -> &Self {
        unsafe { hb::HPDF_Page_MoveToNextLine(self.page) };
        self
    }

    /// Appends a rectangle to the current path.
    ///
    pub fn rectangle(&self, x: f32, y: f32, width: f32, height: f32) -> &Self {
        unsafe { hb::HPDF_Page_Rectangle(self.page, x, y, width, height) };
        self
    }

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
