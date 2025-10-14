use hard_xml::{XmlRead, XmlWrite};

use crate::{__setter, __string_enum, __xml_test_suites};

/// Vertical Merge (Rowspan)
/// Specifies whether this cell is merged vertically with cells above/below
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:vMerge")]
pub struct VMerge {
    #[xml(attr = "w:val")]
    pub val: Option<VMergeType>,
}

impl VMerge {
    __setter!(val: Option<VMergeType>);
}

impl From<VMergeType> for VMerge {
    fn from(val: VMergeType) -> Self {
        VMerge { val: Some(val) }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum VMergeType {
    Restart,  // Start of a merged cell region
    Continue, // Continuation of merged cell region (cell is hidden)
}

impl Default for VMergeType {
    fn default() -> Self {
        VMergeType::Restart
    }
}

__string_enum! {
    VMergeType {
        Restart = "restart",
        Continue = "continue",
    }
}

__xml_test_suites!(
    VMerge,
    VMerge::default(),
    "<w:vMerge/>",
    VMerge::from(VMergeType::Restart),
    r#"<w:vMerge w:val="restart"/>"#,
    VMerge::from(VMergeType::Continue),
    r#"<w:vMerge w:val="continue"/>"#,
);
