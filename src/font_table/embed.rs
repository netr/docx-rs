//! Embedded-font reference elements inside `<w:font>`.
//!
//! OOXML's `CT_Font` type has four optional child elements that tell
//! a reader where to find the actual font binary for each variant:
//!
//! - `<w:embedRegular  r:id="..."/>` — the regular (upright) face
//! - `<w:embedBold     r:id="..."/>` — the bold face
//! - `<w:embedItalic   r:id="..."/>` — the italic face
//! - `<w:embedBoldItalic r:id="..."/>` — the bold-italic face
//!
//! Each `r:id` resolves to a relationship inside
//! `word/_rels/fontTable.xml.rels` whose `Type` is
//! [`crate::schema::SCHEMA_FONT`] and whose `Target` points at the
//! packaged binary (e.g. `fonts/font1.ttf`).
//!
//! The optional `w:fontKey` attribute carries the XOR-obfuscation key
//! for fonts stored in OOXML's `obfuscatedFont` content type.
//! ships fonts in their non-obfuscated form today, so the field is
//! modeled as an `Option<Cow<'a, str>>` and left unset in every
//! document. Readers that don't find the attribute
//! treat the binary as non-obfuscated, which is the behavior we want.

use std::borrow::Cow;

use hard_xml::{XmlRead, XmlWrite};

/// Reference to the regular-weight variant of an embedded font.
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:embedRegular")]
pub struct EmbedRegular<'a> {
    /// Relationship ID pointing at the binary inside
    /// `word/_rels/fontTable.xml.rels`.
    #[xml(default, attr = "r:id")]
    pub id: Cow<'a, str>,
    /// Optional XOR-obfuscation key for the `obfuscatedFont` content
    /// type. Left unset for non-obfuscated binaries.
    #[xml(attr = "w:fontKey")]
    pub font_key: Option<Cow<'a, str>>,
}

impl<'a> EmbedRegular<'a> {
    pub fn new<T: Into<Cow<'a, str>>>(rel_id: T) -> Self {
        EmbedRegular {
            id: rel_id.into(),
            font_key: None,
        }
    }
}

/// Reference to the bold variant of an embedded font.
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:embedBold")]
pub struct EmbedBold<'a> {
    #[xml(default, attr = "r:id")]
    pub id: Cow<'a, str>,
    #[xml(attr = "w:fontKey")]
    pub font_key: Option<Cow<'a, str>>,
}

impl<'a> EmbedBold<'a> {
    pub fn new<T: Into<Cow<'a, str>>>(rel_id: T) -> Self {
        EmbedBold {
            id: rel_id.into(),
            font_key: None,
        }
    }
}

/// Reference to the italic variant of an embedded font.
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:embedItalic")]
pub struct EmbedItalic<'a> {
    #[xml(default, attr = "r:id")]
    pub id: Cow<'a, str>,
    #[xml(attr = "w:fontKey")]
    pub font_key: Option<Cow<'a, str>>,
}

impl<'a> EmbedItalic<'a> {
    pub fn new<T: Into<Cow<'a, str>>>(rel_id: T) -> Self {
        EmbedItalic {
            id: rel_id.into(),
            font_key: None,
        }
    }
}

/// Reference to the bold-italic variant of an embedded font.
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:embedBoldItalic")]
pub struct EmbedBoldItalic<'a> {
    #[xml(default, attr = "r:id")]
    pub id: Cow<'a, str>,
    #[xml(attr = "w:fontKey")]
    pub font_key: Option<Cow<'a, str>>,
}

impl<'a> EmbedBoldItalic<'a> {
    pub fn new<T: Into<Cow<'a, str>>>(rel_id: T) -> Self {
        EmbedBoldItalic {
            id: rel_id.into(),
            font_key: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hard_xml::{XmlRead, XmlWrite};

    // The `__xml_test_suites!` macro only handles one type per
    // invocation (it hard-codes the function name `xml_test_suites`),
    // so we write one manual round-trip test per Embed variant
    // instead. Each assertion matches the macro's shape: serialize →
    // compare string, then parse → compare struct.

    #[test]
    fn embed_regular_round_trips() -> hard_xml::XmlResult<()> {
        let value = EmbedRegular::new("rId1");
        let xml = r#"<w:embedRegular r:id="rId1"/>"#;
        assert_eq!(xml, value.to_string()?);
        assert_eq!(value, EmbedRegular::from_str(xml)?);
        Ok(())
    }

    #[test]
    fn embed_bold_round_trips() -> hard_xml::XmlResult<()> {
        let value = EmbedBold::new("rId2");
        let xml = r#"<w:embedBold r:id="rId2"/>"#;
        assert_eq!(xml, value.to_string()?);
        assert_eq!(value, EmbedBold::from_str(xml)?);
        Ok(())
    }

    #[test]
    fn embed_italic_round_trips() -> hard_xml::XmlResult<()> {
        let value = EmbedItalic::new("rId3");
        let xml = r#"<w:embedItalic r:id="rId3"/>"#;
        assert_eq!(xml, value.to_string()?);
        assert_eq!(value, EmbedItalic::from_str(xml)?);
        Ok(())
    }

    #[test]
    fn embed_bold_italic_round_trips() -> hard_xml::XmlResult<()> {
        let value = EmbedBoldItalic::new("rId4");
        let xml = r#"<w:embedBoldItalic r:id="rId4"/>"#;
        assert_eq!(xml, value.to_string()?);
        assert_eq!(value, EmbedBoldItalic::from_str(xml)?);
        Ok(())
    }
}
