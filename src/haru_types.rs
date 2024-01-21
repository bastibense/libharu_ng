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

/// The error type for libharu.
///
#[derive(Debug)]
pub enum HaruError {
    ArrayCountErr,
    ArrayItemNotFound,
    ArrayItemUnexpectedType,
    BinaryLengthErr,
    CannotGetPallet,
    DictCountErr,
    DictItemNotFound,
    DictItemUnexpectedType,
    DictStreamLengthNotFound,
    DocEncryptDictNotFound,
    DocInvalidObject,
    DuplicateRegistration,
    ExceedJwwCodeNumLimit,
    EncryptInvalidPassword,
    ErrUnknownClass,
    ExceedGstateLimit,
    FaildToAllocMem,
    FileIoError,
    FileOpenError,
    FontExists,
    FontInvalidWidthsTable,
    InvalidAfmHeader,
    InvalidAnnotation,
    InvalidBitPerComponent,
    InvalidCharMatricsData,
    InvalidColorSpace,
    InvalidCompressionMode,
    InvalidDateTime,
    InvalidDestination,
    InvalidDocument,
    InvalidDocumentState,
    InvalidEncoder,
    InvalidEncoderType,
    InvalidEncodingName,
    InvalidEncryptKeyLen,
    InvalidFontdefData,
    InvalidFontdefType,
    InvalidFontName,
    InvalidImage,
    InvalidJpegData,
    InvalidNData,
    InvalidObject,
    InvalidObjId,
    InvalidOperation,
    InvalidOutline,
    InvalidPage,
    InvalidPages,
    InvalidParameter,
    InvalidPngImage,
    InvalidStream,
    MissingFileNameEntry,
    InvalidTtcFile,
    InvalidTtcIndex,
    InvalidWxData,
    ItemNotFound,
    LibpngError,
    NameInvalidValue,
    NameOutOfRange,
    PageInvalidParamCount,
    PagesMissingKidsEntry,
    PageCannotFindObject,
    PageCannotGetRootPages,
    PageCannotRestoreGstate,
    PageCannotSetParent,
    PageFontNotFound,
    PageInvalidFont,
    PageInvalidFontSize,
    PageInvalidGmode,
    PageInvalidIndex,
    PageInvalidRotateValue,
    PageInvalidSize,
    PageInvalidXobject,
    PageOutOfRange,
    RealOutOfRange,
    StreamEof,
    StreamReadlnContinue,
    StringOutOfRange,
    ThisFuncWasSkipped,
    TtfCannotEmbeddingFont,
    TtfInvalidCmap,
    TtfInvalidFomat,
    TtfMissingTable,
    UnsupportedFontType,
    UnsupportedFunc,
    UnsupportedJpegFormat,
    UnsupportedType1Font,
    XrefCountErr,
    ZlibError,
    InvalidPageIndex,
    InvalidUri,
    PageLayoutOutOfRange,
    PageModeOutOfRange,
    PageNumStyleOutOfRange,
    AnnotInvalidIcon,
    AnnotInvalidBorderStyle,
    PageInvalidDirection,
    InvalidFont,
    PageInsufficientSpace,
    PageInvalidDisplayTime,
    PageInvalidTransitionTime,
    InvalidPageSlideshowType,
    ExtGstateOutOfRange,
    InvalidExtGstate,
    ExtGstateReadOnly,
    InvalidU3dData,
    NameCannotGetNames,
    InvalidIccComponentNum,
    PageInvalidBoundary,
    PageInvalidShadingType,
    UnknownError,
}

impl From<u32> for HaruError {
    fn from(code: u32) -> Self {
        match code {
            0x1001 => HaruError::ArrayCountErr,
            0x1002 => HaruError::ArrayItemNotFound,
            0x1003 => HaruError::ArrayItemUnexpectedType,
            0x1004 => HaruError::BinaryLengthErr,
            0x1005 => HaruError::CannotGetPallet,
            0x1007 => HaruError::DictCountErr,
            0x1008 => HaruError::DictItemNotFound,
            0x1009 => HaruError::DictItemUnexpectedType,
            0x100A => HaruError::DictStreamLengthNotFound,
            0x100B => HaruError::DocEncryptDictNotFound,
            0x100C => HaruError::DocInvalidObject,
            0x100E => HaruError::DuplicateRegistration,
            0x100F => HaruError::ExceedJwwCodeNumLimit,
            0x1011 => HaruError::EncryptInvalidPassword,
            0x1013 => HaruError::ErrUnknownClass,
            0x1014 => HaruError::ExceedGstateLimit,
            0x1015 => HaruError::FaildToAllocMem,
            0x1016 => HaruError::FileIoError,
            0x1017 => HaruError::FileOpenError,
            0x1019 => HaruError::FontExists,
            0x101A => HaruError::FontInvalidWidthsTable,
            0x101B => HaruError::InvalidAfmHeader,
            0x101C => HaruError::InvalidAnnotation,
            0x101E => HaruError::InvalidBitPerComponent,
            0x101F => HaruError::InvalidCharMatricsData,
            0x1020 => HaruError::InvalidColorSpace,
            0x1021 => HaruError::InvalidCompressionMode,
            0x1022 => HaruError::InvalidDateTime,
            0x1023 => HaruError::InvalidDestination,
            0x1025 => HaruError::InvalidDocument,
            0x1026 => HaruError::InvalidDocumentState,
            0x1027 => HaruError::InvalidEncoder,
            0x1028 => HaruError::InvalidEncoderType,
            0x102B => HaruError::InvalidEncodingName,
            0x102C => HaruError::InvalidEncryptKeyLen,
            0x102D => HaruError::InvalidFontdefData,
            0x102E => HaruError::InvalidFontdefType,
            0x102F => HaruError::InvalidFontName,
            0x1030 => HaruError::InvalidImage,
            0x1031 => HaruError::InvalidJpegData,
            0x1032 => HaruError::InvalidNData,
            0x1033 => HaruError::InvalidObject,
            0x1034 => HaruError::InvalidObjId,
            0x1035 => HaruError::InvalidOperation,
            0x1036 => HaruError::InvalidOutline,
            0x1037 => HaruError::InvalidPage,
            0x1038 => HaruError::InvalidPages,
            0x1039 => HaruError::InvalidParameter,
            0x103B => HaruError::InvalidPngImage,
            0x103C => HaruError::InvalidStream,
            0x103D => HaruError::MissingFileNameEntry,
            0x103F => HaruError::InvalidTtcFile,
            0x1040 => HaruError::InvalidTtcIndex,
            0x1041 => HaruError::InvalidWxData,
            0x1042 => HaruError::ItemNotFound,
            0x1043 => HaruError::LibpngError,
            0x1044 => HaruError::NameInvalidValue,
            0x1045 => HaruError::NameOutOfRange,
            0x1048 => HaruError::PageInvalidParamCount,
            0x1049 => HaruError::PagesMissingKidsEntry,
            0x104A => HaruError::PageCannotFindObject,
            0x104B => HaruError::PageCannotGetRootPages,
            0x104C => HaruError::PageCannotRestoreGstate,
            0x104D => HaruError::PageCannotSetParent,
            0x104E => HaruError::PageFontNotFound,
            0x104F => HaruError::PageInvalidFont,
            0x1050 => HaruError::PageInvalidFontSize,
            0x1051 => HaruError::PageInvalidGmode,
            0x1052 => HaruError::PageInvalidIndex,
            0x1053 => HaruError::PageInvalidRotateValue,
            0x1054 => HaruError::PageInvalidSize,
            0x1055 => HaruError::PageInvalidXobject,
            0x1056 => HaruError::PageOutOfRange,
            0x1057 => HaruError::RealOutOfRange,
            0x1058 => HaruError::StreamEof,
            0x1059 => HaruError::StreamReadlnContinue,
            0x105B => HaruError::StringOutOfRange,
            0x105C => HaruError::ThisFuncWasSkipped,
            0x105D => HaruError::TtfCannotEmbeddingFont,
            0x105E => HaruError::TtfInvalidCmap,
            0x105F => HaruError::TtfInvalidFomat,
            0x1060 => HaruError::TtfMissingTable,
            0x1061 => HaruError::UnsupportedFontType,
            0x1062 => HaruError::UnsupportedFunc,
            0x1063 => HaruError::UnsupportedJpegFormat,
            0x1064 => HaruError::UnsupportedType1Font,
            0x1065 => HaruError::XrefCountErr,
            0x1066 => HaruError::ZlibError,
            0x1067 => HaruError::InvalidPageIndex,
            0x1068 => HaruError::InvalidUri,
            0x1069 => HaruError::PageLayoutOutOfRange,
            0x1070 => HaruError::PageModeOutOfRange,
            0x1071 => HaruError::PageNumStyleOutOfRange,
            0x1072 => HaruError::AnnotInvalidIcon,
            0x1073 => HaruError::AnnotInvalidBorderStyle,
            0x1074 => HaruError::PageInvalidDirection,
            0x1075 => HaruError::InvalidFont,
            0x1076 => HaruError::PageInsufficientSpace,
            0x1077 => HaruError::PageInvalidDisplayTime,
            0x1078 => HaruError::PageInvalidTransitionTime,
            0x1079 => HaruError::InvalidPageSlideshowType,
            0x1080 => HaruError::ExtGstateOutOfRange,
            0x1081 => HaruError::InvalidExtGstate,
            0x1082 => HaruError::ExtGstateReadOnly,
            0x1083 => HaruError::InvalidU3dData,
            0x1084 => HaruError::NameCannotGetNames,
            0x1085 => HaruError::InvalidIccComponentNum,
            0x1086 => HaruError::PageInvalidBoundary,
            0x1088 => HaruError::PageInvalidShadingType,
            _ => HaruError::UnknownError,
        }
    }
}

/// The LineCap
///
pub enum LineCap {
    /// The line is squared off at the endpoint of the path.
    ButtEnd,
    /// End of line becomes a semicircle whose center is at path endpoint.
    RoundEnd,
    /// Line continues beyond endpoint, goes on half the endpoint stroke width.
    ProjectingSquareEnd,
}

impl LineCap {
    pub fn to_hpdf_line_cap(&self) -> hb::HPDF_LineCap {
        match self {
            LineCap::ButtEnd => hb::_HPDF_LineCap_HPDF_BUTT_END,
            LineCap::RoundEnd => hb::_HPDF_LineCap_HPDF_ROUND_END,
            LineCap::ProjectingSquareEnd => hb::_HPDF_LineCap_HPDF_PROJECTING_SQUARE_END,
        }
    }
}

impl From<hb::HPDF_LineCap> for LineCap {
    fn from(cap: hb::HPDF_LineCap) -> Self {
        match cap {
            hb::_HPDF_LineCap_HPDF_BUTT_END => LineCap::ButtEnd,
            hb::_HPDF_LineCap_HPDF_ROUND_END => LineCap::RoundEnd,
            hb::_HPDF_LineCap_HPDF_PROJECTING_SQUARE_END => LineCap::ProjectingSquareEnd,
            _ => LineCap::ButtEnd,
        }
    }
}

/// The LineJoin
///
pub enum LineJoin {
    /// The outer edges of the strokes for the two segments are extended until they meet at an angle, as in a picture frame.
    MiterJoin,
    /// An arc of a circle with a diameter equal to the line width is drawn around the point where the two segments meet, connecting the outer edges of the strokes for the two segments.
    RoundJoin,
    /// The two segments are finished with butt caps and the resulting notch beyond the ends of the segments is filled with a triangle.
    BevelJoin,
}

impl LineJoin {
    pub fn to_hpdf_line_join(&self) -> hb::HPDF_LineJoin {
        match self {
            LineJoin::MiterJoin => hb::_HPDF_LineJoin_HPDF_MITER_JOIN,
            LineJoin::RoundJoin => hb::_HPDF_LineJoin_HPDF_ROUND_JOIN,
            LineJoin::BevelJoin => hb::_HPDF_LineJoin_HPDF_BEVEL_JOIN,
        }
    }
}

impl From<hb::HPDF_LineJoin> for LineJoin {
    fn from(join: hb::HPDF_LineJoin) -> Self {
        match join {
            hb::_HPDF_LineJoin_HPDF_MITER_JOIN => LineJoin::MiterJoin,
            hb::_HPDF_LineJoin_HPDF_ROUND_JOIN => LineJoin::RoundJoin,
            hb::_HPDF_LineJoin_HPDF_BEVEL_JOIN => LineJoin::BevelJoin,
            _ => LineJoin::MiterJoin,
        }
    }
}

/// The RenderingMode
///
pub enum RenderingMode {
    /// Fill text.
    Fill,
    /// Stroke text.
    Stroke,
    /// Fill, then stroke text.
    FillStroke,
    /// Neither fill nor stroke text (invisible).
    Invisible,
    /// Fill text and add to path for clipping.
    FillClip,
    /// Stroke text and add to path for clipping.
    StrokeClip,
    /// Fill, then stroke text and add to path for clipping.
    FillStrokeClip,
    /// Add text to path for clipping.
    Clip,
}

impl RenderingMode {
    pub fn to_hpdf_rendering_mode(&self) -> hb::HPDF_TextRenderingMode {
        match self {
            RenderingMode::Fill => hb::_HPDF_TextRenderingMode_HPDF_FILL,
            RenderingMode::Stroke => hb::_HPDF_TextRenderingMode_HPDF_STROKE,
            RenderingMode::FillStroke => hb::_HPDF_TextRenderingMode_HPDF_FILL_THEN_STROKE,
            RenderingMode::Invisible => hb::_HPDF_TextRenderingMode_HPDF_INVISIBLE,
            RenderingMode::FillClip => hb::_HPDF_TextRenderingMode_HPDF_FILL_CLIPPING,
            RenderingMode::StrokeClip => hb::_HPDF_TextRenderingMode_HPDF_STROKE_CLIPPING,
            RenderingMode::FillStrokeClip => hb::_HPDF_TextRenderingMode_HPDF_FILL_STROKE_CLIPPING,
            RenderingMode::Clip => hb::_HPDF_TextRenderingMode_HPDF_CLIPPING,
        }
    }
}

impl From<hb::HPDF_TextRenderingMode> for RenderingMode {
    fn from(mode: hb::HPDF_TextRenderingMode) -> Self {
        match mode {
            hb::_HPDF_TextRenderingMode_HPDF_FILL => RenderingMode::Fill,
            hb::_HPDF_TextRenderingMode_HPDF_STROKE => RenderingMode::Stroke,
            hb::_HPDF_TextRenderingMode_HPDF_FILL_THEN_STROKE => RenderingMode::FillStroke,
            hb::_HPDF_TextRenderingMode_HPDF_INVISIBLE => RenderingMode::Invisible,
            hb::_HPDF_TextRenderingMode_HPDF_FILL_CLIPPING => RenderingMode::FillClip,
            hb::_HPDF_TextRenderingMode_HPDF_STROKE_CLIPPING => RenderingMode::StrokeClip,
            hb::_HPDF_TextRenderingMode_HPDF_FILL_STROKE_CLIPPING => RenderingMode::FillStrokeClip,
            hb::_HPDF_TextRenderingMode_HPDF_CLIPPING => RenderingMode::Clip,
            _ => RenderingMode::Fill,
        }
    }
}
