//! This module contains types that are used in the library.
//!

use crate::haru_bindings as hb;

pub enum PageMode {
    /// Display the document with neither outline nor thumbnail.
    UseNone,
    /// Display the document with outline pane.
    UseOutline,
    /// Display the document with thumbnail pane.
    UseThumbs,
    /// Display the document with full screen mode.
    FullScreen,
}

impl PageMode {
    pub fn to_hpdf_mode(&self) -> hb::HPDF_PageMode {
        match self {
            PageMode::UseNone => hb::_HPDF_PageMode_HPDF_PAGE_MODE_USE_NONE,
            PageMode::UseOutline => hb::_HPDF_PageMode_HPDF_PAGE_MODE_USE_OUTLINE,
            PageMode::UseThumbs => hb::_HPDF_PageMode_HPDF_PAGE_MODE_USE_THUMBS,
            PageMode::FullScreen => hb::_HPDF_PageMode_HPDF_PAGE_MODE_FULL_SCREEN,
        }
    }

    pub fn from_hpdf_mode(mode: hb::HPDF_PageMode) -> Self {
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
    pub fn to_hpdf_layout(&self) -> hb::HPDF_PageLayout {
        match self {
            PageLayout::Single => hb::_HPDF_PageLayout_HPDF_PAGE_LAYOUT_SINGLE,
            PageLayout::OneColumn => hb::_HPDF_PageLayout_HPDF_PAGE_LAYOUT_ONE_COLUMN,
            PageLayout::TwoColumnLeft => hb::_HPDF_PageLayout_HPDF_PAGE_LAYOUT_TWO_COLUMN_LEFT,
            PageLayout::TwoColumnRight => hb::_HPDF_PageLayout_HPDF_PAGE_LAYOUT_TWO_COLUMN_RIGHT,
        }
    }

    pub fn from_hpdf_layout(layout: hb::HPDF_PageLayout) -> Self {
        match layout {
            hb::_HPDF_PageLayout_HPDF_PAGE_LAYOUT_SINGLE => PageLayout::Single,
            hb::_HPDF_PageLayout_HPDF_PAGE_LAYOUT_ONE_COLUMN => PageLayout::OneColumn,
            hb::_HPDF_PageLayout_HPDF_PAGE_LAYOUT_TWO_COLUMN_LEFT => PageLayout::TwoColumnLeft,
            hb::_HPDF_PageLayout_HPDF_PAGE_LAYOUT_TWO_COLUMN_RIGHT => PageLayout::TwoColumnRight,
            _ => PageLayout::Single,
        }
    }
}

pub enum InfoAttr {
    CreationDate,
    ModDate,
    Author,
    Creator,
    Title,
    Subject,
    Keywords,
}

impl InfoAttr {
    pub fn to_hpdf_info_attr(&self) -> hb::HPDF_InfoType {
        match self {
            InfoAttr::CreationDate => hb::_HPDF_InfoType_HPDF_INFO_CREATION_DATE,
            InfoAttr::ModDate => hb::_HPDF_InfoType_HPDF_INFO_MOD_DATE,
            InfoAttr::Author => hb::_HPDF_InfoType_HPDF_INFO_AUTHOR,
            InfoAttr::Creator => hb::_HPDF_InfoType_HPDF_INFO_CREATOR,
            InfoAttr::Title => hb::_HPDF_InfoType_HPDF_INFO_TITLE,
            InfoAttr::Subject => hb::_HPDF_InfoType_HPDF_INFO_SUBJECT,
            InfoAttr::Keywords => hb::_HPDF_InfoType_HPDF_INFO_KEYWORDS,
        }
    }
}

pub enum CompressionMode {
    /// No compression.
    None,
    /// Compress the contents stream of the page.
    Text,
    /// Compress the streams of the image objects.
    Image,
    /// Other stream datas (fonts, cmaps and so on) are compressed.
    Metadata,
    /// All stream datas are compressed (HPDF_COMP_TEXT | HPDF_COMP_IMAGE | HPDF_COMP_METADATA).
    All,
}

impl CompressionMode {
    pub fn to_hpdf_compression(&self) -> hb::HPDF_UINT {
        match self {
            CompressionMode::None => hb::HPDF_COMP_NONE,
            CompressionMode::Text => hb::HPDF_COMP_TEXT,
            CompressionMode::Image => hb::HPDF_COMP_IMAGE,
            CompressionMode::Metadata => hb::HPDF_COMP_METADATA,
            CompressionMode::All => hb::HPDF_COMP_ALL,
        }
    }
}
