//! Document background
//!
//! The `<w:background>` element is an optional first child of `<w:document>`
//! that specifies a page-fill color or theme color for the document. See
//! ECMA-376 §17.2.1 (background).
//!
//! Example:
//! ```xml
//! <w:document …>
//!   <w:background w:color="00B050"/>
//!   <w:body>…</w:body>
//! </w:document>
//! ```

use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::formatting::ThemeColor;

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:background")]
pub struct Background<'a> {
    /// Page background color as written in the OOXML `w:color` attribute
    /// (typically a 6-character hex like `00B050`, or the sentinel `auto`).
    /// No validation or normalization is performed here.
    #[xml(attr = "w:color")]
    pub color: Option<Cow<'a, str>>,
    /// Theme color reference (e.g. `background1`), used when the color
    /// derives from the document theme instead of an explicit hex.
    #[xml(attr = "w:themeColor")]
    pub theme_color: Option<ThemeColor>,
    /// Optional theme tint applied to the theme color.
    #[xml(attr = "w:themeTint")]
    pub theme_tint: Option<Cow<'a, str>>,
    /// Optional theme shade applied to the theme color.
    #[xml(attr = "w:themeShade")]
    pub theme_shade: Option<Cow<'a, str>>,
}
