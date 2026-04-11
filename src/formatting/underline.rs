use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::__xml_test_suites;

/// Underline
///
/// ```rust
/// use docx_rust::formatting::*;
///
/// let udl = Underline::from("00ff00");
/// let udl = Underline::from(String::from("ff0000"));
/// let udl = Underline::from(("00ff00", UnderlineStyle::Dash));
/// let udl = Underline::from((String::from("ff0000"), UnderlineStyle::DotDash));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:u")]
pub struct Underline<'a> {
    #[xml(attr = "w:color")]
    pub color: Option<Cow<'a, str>>,
    /// Underline style - accepts any string value from the DOCX
    #[xml(attr = "w:val")]
    pub val: Option<Cow<'a, str>>,
}

impl From<String> for Underline<'_> {
    fn from(val: String) -> Self {
        Underline {
            color: Some(val.into()),
            val: None,
        }
    }
}

impl<'a> From<&'a str> for Underline<'a> {
    fn from(val: &'a str) -> Self {
        Underline {
            color: Some(val.into()),
            val: None,
        }
    }
}

impl From<UnderlineStyle> for Underline<'_> {
    fn from(val: UnderlineStyle) -> Self {
        Underline {
            color: None,
            val: Some(val.to_string().into()),
        }
    }
}

impl From<(String, UnderlineStyle)> for Underline<'_> {
    fn from(val: (String, UnderlineStyle)) -> Self {
        Underline {
            color: Some(val.0.into()),
            val: Some(val.1.to_string().into()),
        }
    }
}

impl<'a> From<(&'a str, UnderlineStyle)> for Underline<'a> {
    fn from(val: (&'a str, UnderlineStyle)) -> Self {
        Underline {
            color: Some(val.0.into()),
            val: Some(val.1.to_string().into()),
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum UnderlineStyle {
    Dash,
    DashDotDotHeavy,
    DashDotHeavy,
    DashedHeavy,
    DashLong,
    DashLongHeavy,
    DotDash,
    DotDotDash,
    Dotted,
    DottedHeavy,
    Double,
    None,
    Single,
    Thick,
    Wave,
    WavyDouble,
    WavyHeavy,
    Words,
}

impl std::fmt::Display for UnderlineStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            UnderlineStyle::Dash => write!(f, "dash"),
            UnderlineStyle::DashDotDotHeavy => write!(f, "dashDotDotHeavy"),
            UnderlineStyle::DashDotHeavy => write!(f, "dashDotHeavy"),
            UnderlineStyle::DashedHeavy => write!(f, "dashedHeavy"),
            UnderlineStyle::DashLong => write!(f, "dashLong"),
            UnderlineStyle::DashLongHeavy => write!(f, "dashLongHeavy"),
            UnderlineStyle::DotDash => write!(f, "dotDash"),
            UnderlineStyle::DotDotDash => write!(f, "dotDotDash"),
            UnderlineStyle::Dotted => write!(f, "dotted"),
            UnderlineStyle::DottedHeavy => write!(f, "dottedHeavy"),
            UnderlineStyle::Double => write!(f, "double"),
            UnderlineStyle::None => write!(f, "none"),
            UnderlineStyle::Single => write!(f, "single"),
            UnderlineStyle::Thick => write!(f, "thick"),
            UnderlineStyle::Wave => write!(f, "wave"),
            UnderlineStyle::WavyDouble => write!(f, "wavyDouble"),
            UnderlineStyle::WavyHeavy => write!(f, "wavyHeavy"),
            UnderlineStyle::Words => write!(f, "words"),
        }
    }
}

__xml_test_suites!(
    Underline,
    Underline::default(),
    r#"<w:u/>"#,
    Underline::from("00ff00"),
    r#"<w:u w:color="00ff00"/>"#,
    Underline::from(String::from("ff0000")),
    r#"<w:u w:color="ff0000"/>"#,
    Underline::from(("00ff00", UnderlineStyle::Dash)),
    r#"<w:u w:color="00ff00" w:val="dash"/>"#,
    Underline::from((String::from("ff0000"), UnderlineStyle::DotDash)),
    r#"<w:u w:color="ff0000" w:val="dotDash"/>"#,
);
