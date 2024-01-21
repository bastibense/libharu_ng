use crate::haru_bindings as hb;
use crate::page::PdfPage;

/// The PDF document.
///
pub struct PdfDocument {
    /// The reference to the haru document.
    pub doc: hb::HPDF_Doc,
}

impl PdfDocument {
    /// Create an instance of a document object and initialize it.
    ///
    pub fn new() -> Self {
        let doc = unsafe { hb::HPDF_New(None, core::ptr::null_mut()) };
        Self { doc }
    }

    /// HPDF_SetPageMode() sets how the document should be displayed.
    ///
    pub fn set_page_mode(&self, mode: PageMode) {
        unsafe { hb::HPDF_SetPageMode(self.doc, mode.to_hpdf_mode()) };
    }

    /// HPDF_GetPageLayout() returns the current setting for page mode.
    ///
    pub fn get_page_mode(&self) -> PageMode {
        let mode = unsafe { hb::HPDF_GetPageMode(self.doc) };
        PageMode::from_hpdf_mode(mode)
    }

    /// HPDF_SetPageLayout() sets how the page should be displayed. If this attribute
    /// is not set, the setting of the viewer application is used.
    ///
    pub fn set_page_layout(&self, layout: PageLayout) {
        unsafe { hb::HPDF_SetPageLayout(self.doc, layout.to_hpdf_layout()) };
    }

    /// HPDF_GetPageLayout() returns the current setting for page layout.
    ///
    pub fn get_page_layout(&self) -> PageLayout {
        let layout = unsafe { hb::HPDF_GetPageLayout(self.doc) };
        PageLayout::from_hpdf_layout(layout)
    }

    /// Save the PDF document to a file.
    ///
    pub fn save_to_file(&self, filename: &str) {
        let filename = std::ffi::CString::new(filename).unwrap();
        unsafe { hb::HPDF_SaveToFile(self.doc, filename.as_ptr()) };
    }

    /// Returns the last error code of specified document object.
    ///
    pub fn get_error(&self) -> hb::HPDF_STATUS {
        unsafe { hb::HPDF_GetError(self.doc) }
    }

    /// Create a new page.
    ///
    pub fn add_page(&self) -> PdfPage {
        let page: *mut hb::_HPDF_Dict_Rec = unsafe { hb::HPDF_AddPage(self.doc) };
        PdfPage { page }
    }

    /// Set the title of the document.
    ///
    pub fn set_title(&self, title: &str) {
        let title = std::ffi::CString::new(title).unwrap();
        unsafe {
            hb::HPDF_SetInfoAttr(self.doc, hb::_HPDF_InfoType_HPDF_INFO_TITLE, title.as_ptr())
        };
    }
}

impl Drop for PdfDocument {
    fn drop(&mut self) {
        unsafe { hb::HPDF_Free(self.doc) };
    }
}

pub enum PageMode {
    UseNone,
    UseOutline,
    UseThumbs,
    FullScreen,
}

impl PageMode {
    fn to_hpdf_mode(&self) -> hb::HPDF_PageMode {
        match self {
            PageMode::UseNone => hb::_HPDF_PageMode_HPDF_PAGE_MODE_USE_NONE,
            PageMode::UseOutline => hb::_HPDF_PageMode_HPDF_PAGE_MODE_USE_OUTLINE,
            PageMode::UseThumbs => hb::_HPDF_PageMode_HPDF_PAGE_MODE_USE_THUMBS,
            PageMode::FullScreen => hb::_HPDF_PageMode_HPDF_PAGE_MODE_FULL_SCREEN,
        }
    }

    fn from_hpdf_mode(mode: hb::HPDF_PageMode) -> Self {
        match mode {
            hb::_HPDF_PageMode_HPDF_PAGE_MODE_USE_NONE => PageMode::UseNone,
            hb::_HPDF_PageMode_HPDF_PAGE_MODE_USE_OUTLINE => PageMode::UseOutline,
            hb::_HPDF_PageMode_HPDF_PAGE_MODE_USE_THUMBS => PageMode::UseThumbs,
            hb::_HPDF_PageMode_HPDF_PAGE_MODE_FULL_SCREEN => PageMode::FullScreen,
            _ => PageMode::UseNone,
        }
    }
}

pub enum PageLayout {
    /// Only one page is displayed.
    Single,
    /// Display the pages in one column.
    OneColumn,
    /// Display in two columns. Odd page number is displayed left.
    TwoColumnLeft,
    ///  Display in two columns. Odd page number is displayed right.
    TwoColumnRight,
}

impl PageLayout {
    fn to_hpdf_layout(&self) -> hb::HPDF_PageLayout {
        match self {
            PageLayout::Single => hb::_HPDF_PageLayout_HPDF_PAGE_LAYOUT_SINGLE,
            PageLayout::OneColumn => hb::_HPDF_PageLayout_HPDF_PAGE_LAYOUT_ONE_COLUMN,
            PageLayout::TwoColumnLeft => hb::_HPDF_PageLayout_HPDF_PAGE_LAYOUT_TWO_COLUMN_LEFT,
            PageLayout::TwoColumnRight => hb::_HPDF_PageLayout_HPDF_PAGE_LAYOUT_TWO_COLUMN_RIGHT,
        }
    }

    fn from_hpdf_layout(layout: hb::HPDF_PageLayout) -> Self {
        match layout {
            hb::_HPDF_PageLayout_HPDF_PAGE_LAYOUT_SINGLE => PageLayout::Single,
            hb::_HPDF_PageLayout_HPDF_PAGE_LAYOUT_ONE_COLUMN => PageLayout::OneColumn,
            hb::_HPDF_PageLayout_HPDF_PAGE_LAYOUT_TWO_COLUMN_LEFT => PageLayout::TwoColumnLeft,
            hb::_HPDF_PageLayout_HPDF_PAGE_LAYOUT_TWO_COLUMN_RIGHT => PageLayout::TwoColumnRight,
            _ => PageLayout::Single,
        }
    }
}
