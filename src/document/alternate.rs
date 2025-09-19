use hard_xml::{XmlRead, XmlWrite};

use super::drawing::Drawing;

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "mc:AlternateContent")]
pub struct AlternateContent<'a> {
    #[xml(child = "mc:Choice")]
    pub choices: Vec<AlternateContentChoice<'a>>,
    #[xml(child = "mc:Fallback")]
    pub fallback: Option<AlternateContentFallback<'a>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "mc:Choice")]
pub struct AlternateContentChoice<'a> {
    #[xml(attr = "Requires")]
    pub requires: Option<std::borrow::Cow<'a, str>>,
    #[xml(child = "w:drawing")]
    pub drawings: Vec<Drawing<'a>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "mc:Fallback")]
pub struct AlternateContentFallback<'a> {
    #[xml(child = "w:drawing")]
    pub drawings: Vec<Drawing<'a>>,
}
