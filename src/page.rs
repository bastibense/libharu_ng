//!  PDF-Page related structs and functions.
//!
//!
//! ## Implmeneation Status
//!
//! - [x] HPDF_Page_Arc()
//! - [x] HPDF_Page_BeginText()
//! - [x] HPDF_Page_Circle()
//! - [x] HPDF_Page_Clip()
//! - [x] HPDF_Page_ClosePath()
//! - [x] HPDF_Page_ClosePathStroke()
//! - [x] HPDF_Page_ClosePathEofillStroke()
//! - [x] HPDF_Page_ClosePathFillStroke()
//! - [x] HPDF_Page_Concat()
//! - [x] HPDF_Page_CurveTo()
//! - [x] HPDF_Page_CurveTo2()
//! - [x] HPDF_Page_CurveTo3()
//! - [ ] HPDF_Page_DrawImage()
//! - [x] HPDF_Page_Ellipse()
//! - [x] HPDF_Page_EndPath()
//! - [x] HPDF_Page_EndText()
//! - [x] HPDF_Page_Eoclip()
//! - [x] HPDF_Page_Eofill()
//! - [x] HPDF_Page_EofillStroke()
//! - [ ] HPDF_Page_ExecuteXObject()
//! - [x] HPDF_Page_Fill()
//! - [x] HPDF_Page_FillStroke()
//! - [x] HPDF_Page_GRestore()
//! - [x] HPDF_Page_GSave()
//! - [x] HPDF_Page_LineTo()
//! - [x] HPDF_Page_MoveTextPos()
//! - [x] HPDF_Page_MoveTextPos2()
//! - [x] HPDF_Page_MoveTo()
//! - [x] HPDF_Page_MoveToNextLine()
//! - [x] HPDF_Page_Rectangle()
//! - [x] HPDF_Page_SetCharSpace()
//! - [x] HPDF_Page_SetCMYKFill()
//! - [x] HPDF_Page_SetCMYKStroke()
//! - [ ] HPDF_Page_SetDash()
//! - [ ] HPDF_Page_SetExtGState()
//! - [ ] HPDF_Page_SetFontAndSize()
//! - [x] HPDF_Page_SetGrayFill()
//! - [x] HPDF_Page_SetGrayStroke()
//! - [x] HPDF_Page_SetHorizontalScalling()
//! - [x] HPDF_Page_SetLineCap()
//! - [x] HPDF_Page_SetLineJoin()
//! - [x] HPDF_Page_SetLineWidth()
//! - [x] HPDF_Page_SetMiterLimit()
//! - [x] HPDF_Page_SetRGBFill()
//! - [x] HPDF_Page_SetRGBStroke()
//! - [x] HPDF_Page_SetTextLeading()
//! - [x] HPDF_Page_SetTextMatrix()
//! - [x] HPDF_Page_SetTextRenderingMode()
//! - [x] HPDF_Page_SetTextRise()
//! - [x] HPDF_Page_SetWordSpace()
//! - [x] HPDF_Page_ShowText()
//! - [x] HPDF_Page_ShowTextNextLine()
//! - [x] HPDF_Page_ShowTextNextLineEx()
//! - [x] HPDF_Page_Stroke()
//! - [x] HPDF_Page_TextOut()
//! - [ ] HPDF_Page_TextRect()

use haru_types::LineCap;
use haru_types::RenderingMode;

use crate::haru_bindings as hb;
use crate::haru_types;

/// The PDF Page API.
///
pub struct PdfPage {
    /// The reference to the haru page.
    pub page: hb::HPDF_Page,
}

impl PdfPage {
    /// arc() appends a circle arc to the current path. Angles
    /// are given in degrees, with 0 degrees being vertical, upward, from
    /// the (x,y) position.
    ///
    /// API: HPDF_Page_Arc
    ///
    pub fn arc(&self, x: f32, y: f32, radius: f32, ang1: f32, ang2: f32) -> &Self {
        unsafe { hb::HPDF_Page_Arc(self.page, x, y, radius, ang1, ang2) };
        self
    }

    /// begin_text() begins a text object and sets the text position to (0, 0).
    ///
    /// API: HPDF_Page_BeginText
    ///
    pub fn begin_text(&self) -> &Self {
        unsafe { hb::HPDF_Page_BeginText(self.page) };
        self
    }

    /// circle() appends a circle to the current path.
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
    /// API: HPDF_Page_Clip
    ///
    pub fn clip(&self) -> &Self {
        unsafe { hb::HPDF_Page_Clip(self.page) };
        self
    }

    /// close_path() appends a straight line from the current point to the
    /// start point of sub path. The current point is moved to the start point of
    /// sub path.
    ///
    /// API: HPDF_Page_ClosePath
    ///
    pub fn close_path(&self) -> &Self {
        unsafe { hb::HPDF_Page_ClosePath(self.page) };
        self
    }

    /// close_path_stroke() closes the current path. Then, it paints the path.
    ///
    /// API: HPDF_Page_ClosePathStroke
    ///
    pub fn close_path_stroke(&self) -> &Self {
        unsafe { hb::HPDF_Page_ClosePathStroke(self.page) };
        self
    }

    /// close_path_eofill_stroke() closes the current path, fills the current
    /// path using the even-odd rule, then paints the path.
    ///
    /// API: HPDF_Page_ClosePathEofillStroke
    ///
    pub fn close_path_eofill_stroke(&self) -> &Self {
        unsafe { hb::HPDF_Page_ClosePathEofillStroke(self.page) };
        self
    }

    /// close_path_fill_stroke() closes the current path, fills the current
    /// path using the nonzero winding number rule, then paints the path.
    ///
    /// API: HPDF_Page_ClosePathFillStroke
    ///
    pub fn close_path_fill_stroke(&self) -> &Self {
        unsafe { hb::HPDF_Page_ClosePathFillStroke(self.page) };
        self
    }

    /// concat() concatenates the page's current transformation matrix and specified matrix.
    ///
    /// API: HPDF_Page_Concat
    ///
    pub fn concat(&self, a: f32, b: f32, c: f32, d: f32, x: f32, y: f32) -> &Self {
        unsafe { hb::HPDF_Page_Concat(self.page, a, b, c, d, x, y) };
        self
    }

    /// curve_to() appends a Bézier curve to the current path using the control points (x1, y1)
    /// and (x2, y2) and (x3, y3), then sets the current point to (x3, y3).
    ///
    /// API: HPDF_Page_CurveTo
    ///
    pub fn curve_to(&self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> &Self {
        unsafe { hb::HPDF_Page_CurveTo(self.page, x1, y1, x2, y2, x3, y3) };
        self
    }

    /// curve_to2() appends a Bézier curve to the current path using the current point and
    /// (x2, y2) and (x3, y3) as control points. Then, the current point is set to (x3, y3).
    ///
    /// API: HPDF_Page_CurveTo2
    ///
    pub fn curve_to2(&self, x2: f32, y2: f32, x3: f32, y3: f32) -> &Self {
        unsafe { hb::HPDF_Page_CurveTo2(self.page, x2, y2, x3, y3) };
        self
    }

    /// curve_to3() appends a Bézier curve to the current path using two spesified points.
    /// The point (x1, y1) and the point (x3, y3) are used as the control points for a Bézier curve
    /// and current point is moved to the point (x3, y3)
    ///
    /// API: HPDF_Page_CurveTo3
    ///
    pub fn curve_to3(&self, x1: f32, y1: f32, x3: f32, y3: f32) -> &Self {
        unsafe { hb::HPDF_Page_CurveTo3(self.page, x1, y1, x3, y3) };
        self
    }

    /// ellipse() appends an ellipse to the current path.
    ///
    /// API: HPDF_Page_Ellipse
    ///
    pub fn ellipse(&self, x: f32, y: f32, xray: f32, yray: f32) -> &Self {
        unsafe { hb::HPDF_Page_Ellipse(self.page, x, y, xray, yray) };
        self
    }

    /// end_path() ends the path object without filling or painting.
    ///
    /// API: HPDF_Page_EndPath
    ///
    pub fn end_path(&self) -> &Self {
        unsafe { hb::HPDF_Page_EndPath(self.page) };
        self
    }

    /// end_text() ends a text object.
    ///
    /// API: HPDF_Page_EndText
    ///
    pub fn end_text(&self) -> &Self {
        unsafe { hb::HPDF_Page_EndText(self.page) };
        self
    }

    /// eo_clip() modifies the current clipping path by intersecting it
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
    /// API: HPDF_Page_Eoclip
    ///
    pub fn eo_clip(&self) -> &Self {
        unsafe { hb::HPDF_Page_Eoclip(self.page) };
        self
    }

    /// eo_fill() fills the current path using the even-odd rule.
    ///
    /// API: HPDF_Page_Eofill
    ///
    pub fn eo_fill(&self) -> &Self {
        unsafe { hb::HPDF_Page_Eofill(self.page) };
        self
    }

    /// eo_fill_stroke() fills the current path using the even-odd rule, then paints the path.
    ///
    /// API: HPDF_Page_EofillStroke
    ///
    pub fn eo_fill_stroke(&self) -> &Self {
        unsafe { hb::HPDF_Page_EofillStroke(self.page) };
        self
    }

    /// fill() fills the current path using the nonzero winding number rule.
    ///
    /// API: HPDF_Page_Fill
    ///
    pub fn fill(&self) -> &Self {
        unsafe { hb::HPDF_Page_Fill(self.page) };
        self
    }

    /// fill_stroke() fills the current path using the nonzero winding number rule, then paints the path.
    ///
    /// API: HPDF_Page_FillStroke
    ///
    pub fn fill_stroke(&self) -> &Self {
        unsafe { hb::HPDF_Page_FillStroke(self.page) };
        self
    }

    /// g_restore() restore the graphics state which is saved by HPDF_Page_GSave().
    ///
    /// API: HPDF_Page_GRestore
    ///
    pub fn g_restore(&self) -> &Self {
        unsafe { hb::HPDF_Page_GRestore(self.page) };
        self
    }

    /// g_save() saves the page's current graphics parameter to the stack.
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
    /// API: HPDF_Page_GSave
    ///
    pub fn g_save(&self) -> &Self {
        unsafe { hb::HPDF_Page_GSave(self.page) };
        self
    }

    /// line_to() appends a path from the current point to the specified point.
    ///
    /// API: HPDF_Page_LineTo
    ///
    pub fn line_to(&self, x: f32, y: f32) -> &Self {
        unsafe { hb::HPDF_Page_LineTo(self.page, x, y) };
        self
    }

    /// set_char_space() sets the character spacing for text.
    ///
    /// API: HPDF_Page_SetCharSpace
    ///
    pub fn set_char_space(&self, value: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetCharSpace(self.page, value) };
        self
    }

    /// set_cmyk_fill() sets the filling color.
    ///
    /// API: HPDF_Page_SetCMYKFill
    ///
    pub fn set_cmyk_fill(&self, c: f32, m: f32, y: f32, k: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetCMYKFill(self.page, c, m, y, k) };
        self
    }

    /// set_cmyk_stroke() sets the stroking color.
    ///
    /// API: HPDF_Page_SetCMYKStroke
    ///
    pub fn set_cmyk_stroke(&self, c: f32, m: f32, y: f32, k: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetCMYKStroke(self.page, c, m, y, k) };
        self
    }

    /// HPDF_Page_SetGrayFill() sets the filling color.
    ///
    /// API: HPDF_Page_SetGrayFill
    ///
    pub fn set_gray_fill(&self, gray: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetGrayFill(self.page, gray) };
        self
    }

    /// HPDF_Page_SetGrayStroke() sets the stroking color.
    ///
    /// API: HPDF_Page_SetGrayStroke
    ///
    pub fn set_gray_stroke(&self, gray: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetGrayStroke(self.page, gray) };
        self
    }

    /// set_horizontal_scalling() sets the horizontal scalling (scaling) for text showing.
    ///
    /// API: HPDF_Page_SetHorizontalScalling
    ///
    pub fn set_horizontal_scalling(&self, value: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetHorizontalScalling(self.page, value) };
        self
    }

    /// HPDF_Page_SetLineCap() sets the shape to be used at the ends of lines.
    ///
    /// API: HPDF_Page_SetLineCap
    ///
    pub fn set_line_cap(&self, line_cap: LineCap) -> &Self {
        unsafe { hb::HPDF_Page_SetLineCap(self.page, line_cap as u32) };
        self
    }

    /// set_line_join() Sets the line join style in the page.
    ///
    /// API: HPDF_Page_SetLineJoin
    ///
    pub fn set_line_join(&self, line_join: haru_types::LineJoin) -> &Self {
        unsafe { hb::HPDF_Page_SetLineJoin(self.page, line_join as u32) };
        self
    }

    /// set_miter_limit() sets the miter limit for the page.
    ///
    /// API: HPDF_Page_SetMiterLimit
    ///
    pub fn set_miter_limit(&self, miter_limit: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetMiterLimit(self.page, miter_limit) };
        self
    }

    /// set_line_width() sets the width of the line used to stroke a path.
    ///
    /// API: HPDF_Page_SetLineWidth
    ///
    pub fn set_line_width(&self, line_width: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetLineWidth(self.page, line_width) };
        self
    }

    /// move_text_pos() changes the current text position, using the specified offset values.
    /// If the current text position is (x1, y1), the new text position
    /// will be (x1 + x, y1 + y).
    ///
    /// API: HPDF_Page_MoveTextPos
    ///
    pub fn move_text_pos(&self, x: f32, y: f32) -> &Self {
        unsafe { hb::HPDF_Page_MoveTextPos(self.page, x, y) };
        self
    }

    /// move_text_pos2() changes the current text position, using the specified offset values.
    /// If the current text position is (x1, y1), the new text position will be (x1 + x, y1 + y).
    ///
    /// API: HPDF_Page_MoveTextPos2
    ///
    pub fn move_text_pos2(&self, x: f32, y: f32) -> &Self {
        unsafe { hb::HPDF_Page_MoveTextPos2(self.page, x, y) };
        self
    }

    /// move_to() starts a new subpath and move the current point for drawing path,
    /// move_to() sets the start point for the path to the point (x, y).
    ///
    /// API: HPDF_Page_MoveTo
    ///
    pub fn move_to(&self, x: f32, y: f32) -> &Self {
        unsafe { hb::HPDF_Page_MoveTo(self.page, x, y) };
        self
    }

    /// move_to_next_line() moves current position for the text showing depending on
    /// current text showing point and text leading. The new position is calculated
    /// with current text transition matrix.
    ///
    /// API: HPDF_Page_MoveToNextLine
    ///
    pub fn move_to_next_line(&self) -> &Self {
        unsafe { hb::HPDF_Page_MoveToNextLine(self.page) };
        self
    }

    /// rectangle() appends a rectangle to the current path.
    ///
    /// API: HPDF_Page_Rectangle
    ///
    pub fn rectangle(&self, x: f32, y: f32, width: f32, height: f32) -> &Self {
        unsafe { hb::HPDF_Page_Rectangle(self.page, x, y, width, height) };
        self
    }

    /// set_width() sets the width of the page.
    ///
    /// API: HPDF_Page_SetWidth
    ///
    pub fn set_width(&self, width: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetWidth(self.page, width) };
        self
    }

    /// set_height() sets the height of the page.
    ///
    /// API: HPDF_Page_SetHeight
    ///
    pub fn set_height(&self, height: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetHeight(self.page, height) };
        self
    }

    /// HPDF_Page_SetTextRenderingMode() sets the text rendering mode.
    /// The initial value of text rendering mode is HPDF_FILL.
    ///
    /// API: HPDF_Page_SetTextRenderingMode
    ///
    pub fn set_text_rendering_mode(&self, mode: RenderingMode) -> &Self {
        unsafe { hb::HPDF_Page_SetTextRenderingMode(self.page, mode as u32) };
        self
    }

    /// get_width() returns the width of the page.
    ///
    /// API: HPDF_Page_GetWidth
    ///
    pub fn get_width(&self) -> f32 {
        unsafe { hb::HPDF_Page_GetWidth(self.page) }
    }

    /// get_height() returns the height of the page.
    ///   
    /// API: HPDF_Page_GetHeight
    ///
    pub fn get_height(&self) -> f32 {
        unsafe { hb::HPDF_Page_GetHeight(self.page) }
    }

    /// set_rgb_fill() sets the filling color.
    ///
    /// API: HPDF_Page_SetRGBFill
    ///
    pub fn set_rgb_fill(&self, r: f32, g: f32, b: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetRGBFill(self.page, r, g, b) };
        self
    }

    /// set_rgb_stroke() sets the stroking color.
    ///
    /// API: HPDF_Page_SetRGBStroke
    ///
    pub fn set_rgb_stroke(&self, r: f32, g: f32, b: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetRGBStroke(self.page, r, g, b) };
        self
    }

    /// set_text_leading() sets the text leading (line spacing) for
    /// text showing.
    ///
    /// API: HPDF_Page_SetTextLeading
    ///
    pub fn set_text_leading(&self, value: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetTextLeading(self.page, value) };
        self
    }

    /// set_text_matrix sets a transformation matrix for text to be
    /// drawn in using HPDF_Page_ShowText().
    ///
    /// API: HPDF_Page_SetTextMatrix
    ///
    pub fn set_text_matrix(&self, a: f32, b: f32, c: f32, d: f32, x: f32, y: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetTextMatrix(self.page, a, b, c, d, x, y) };
        self
    }

    /// set_text_rise() moves the text position in vertical direction
    /// by the amount of value. Useful for making subscripts or superscripts.
    ///
    /// API: HPDF_Page_SetTextRise
    ///
    pub fn set_text_rise(&self, value: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetTextRise(self.page, value) };
        self
    }

    /// set_word_space() sets the word spacing for text.
    ///
    /// API: HPDF_Page_SetWordSpace
    ///
    pub fn set_word_space(&self, value: f32) -> &Self {
        unsafe { hb::HPDF_Page_SetWordSpace(self.page, value) };
        self
    }

    /// show_text() prints the text at the current position on the page.
    ///
    /// API: HPDF_Page_ShowText
    ///
    pub fn show_text(&self, text: &str) -> &Self {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { hb::HPDF_Page_ShowText(self.page, text.as_ptr()) };
        self
    }

    /// show_text_next_line() moves the current text position to the
    /// start of the next line, then prints the text at the current position on the page.
    ///
    /// API: HPDF_Page_ShowTextNextLine
    ///
    pub fn show_text_next_line(&self, text: &str) -> &Self {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { hb::HPDF_Page_ShowTextNextLine(self.page, text.as_ptr()) };
        self
    }

    /// show_text_next_line_ex() moves the current text position to the start of
    /// the next line; then sets word spacing and character spacing; finally prints
    /// the text at the current position on the page.
    ///
    /// API: HPDF_Page_ShowTextNextLineEx
    ///
    pub fn show_text_next_line_ex(&self, word_space: f32, char_space: f32, text: &str) -> &Self {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            hb::HPDF_Page_ShowTextNextLineEx(self.page, word_space, char_space, text.as_ptr())
        };
        self
    }

    /// stroke() paints the current path.
    ///
    /// API: HPDF_Page_Stroke
    ///
    pub fn stroke(&self) -> &Self {
        unsafe { hb::HPDF_Page_Stroke(self.page) };
        self
    }

    /// text_out() prints the text on the specified position.
    ///
    /// API: HPDF_Page_TextOut
    ///
    pub fn text_out(&self, x: f32, y: f32, text: &str) -> &Self {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { hb::HPDF_Page_TextOut(self.page, x, y, text.as_ptr()) };
        self
    }
}
