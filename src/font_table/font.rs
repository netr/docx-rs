use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{
    __setter, __xml_test_suites,
    font_table::{Charset, EmbedBold, EmbedBoldItalic, EmbedItalic, EmbedRegular, Family, Pitch},
};

/// Font
///
/// ```rust
/// use docx_rust::font_table::*;
///
/// let font = Font::new("Arial")
///     .charset("00")
///     .family("swiss")
///     .pitch("variable");
/// ```
///
/// Embedded font binaries are referenced via the four optional
/// `embed_*` children. Each takes a relationship ID that points at a
/// binary inside `word/_rels/fontTable.xml.rels`:
///
/// ```rust
/// use docx_rust::font_table::*;
///
/// let embedded = Font::new("EB Garamond")
///     .embed_regular(EmbedRegular::new("rId1"))
///     .embed_bold(EmbedBold::new("rId2"));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:font")]
pub struct Font<'a> {
    #[xml(default, attr = "w:name")]
    pub name: Cow<'a, str>,
    #[xml(child = "w:charset")]
    pub charset: Option<Charset<'a>>,
    #[xml(child = "w:family")]
    pub family: Option<Family<'a>>,
    #[xml(child = "w:pitch")]
    pub pitch: Option<Pitch<'a>>,
    #[xml(child = "w:embedRegular")]
    pub embed_regular: Option<EmbedRegular<'a>>,
    #[xml(child = "w:embedBold")]
    pub embed_bold: Option<EmbedBold<'a>>,
    #[xml(child = "w:embedItalic")]
    pub embed_italic: Option<EmbedItalic<'a>>,
    #[xml(child = "w:embedBoldItalic")]
    pub embed_bold_italic: Option<EmbedBoldItalic<'a>>,
}

impl<'a> Font<'a> {
    __setter!(charset: Option<Charset<'a>>);
    __setter!(family: Option<Family<'a>>);
    __setter!(pitch: Option<Pitch<'a>>);
    __setter!(embed_regular: Option<EmbedRegular<'a>>);
    __setter!(embed_bold: Option<EmbedBold<'a>>);
    __setter!(embed_italic: Option<EmbedItalic<'a>>);
    __setter!(embed_bold_italic: Option<EmbedBoldItalic<'a>>);

    pub fn new<T: Into<Cow<'a, str>>>(name: T) -> Self {
        Font {
            name: name.into(),
            ..Default::default()
        }
    }
}

impl<'a, T: Into<Cow<'a, str>>> From<T> for Font<'a> {
    fn from(val: T) -> Self {
        Font::new(val)
    }
}

__xml_test_suites!(
    Font,
    Font::new("Arial"),
    r#"<w:font w:name="Arial"/>"#,
    Font::new("Arial").charset("00"),
    r#"<w:font w:name="Arial"><w:charset w:val="00"/></w:font>"#,
    Font::new("Arial").family("swiss"),
    r#"<w:font w:name="Arial"><w:family w:val="swiss"/></w:font>"#,
    Font::new("Arial").pitch("variable"),
    r#"<w:font w:name="Arial"><w:pitch w:val="variable"/></w:font>"#,
    Font::new("EB Garamond").embed_regular(EmbedRegular::new("rId1")),
    r#"<w:font w:name="EB Garamond"><w:embedRegular r:id="rId1"/></w:font>"#,
    Font::new("EB Garamond")
        .embed_regular(EmbedRegular::new("rId1"))
        .embed_bold(EmbedBold::new("rId2"))
        .embed_italic(EmbedItalic::new("rId3"))
        .embed_bold_italic(EmbedBoldItalic::new("rId4")),
    r#"<w:font w:name="EB Garamond"><w:embedRegular r:id="rId1"/><w:embedBold r:id="rId2"/><w:embedItalic r:id="rId3"/><w:embedBoldItalic r:id="rId4"/></w:font>"#,
);
