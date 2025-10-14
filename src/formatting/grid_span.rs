use hard_xml::{XmlRead, XmlWrite};

use crate::{__setter, __xml_test_suites};

/// Grid Span (Colspan)
/// Specifies the number of columns this cell spans horizontally
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:gridSpan")]
pub struct GridSpan {
    #[xml(attr = "w:val")]
    pub val: Option<usize>,
}

impl GridSpan {
    __setter!(val: Option<usize>);
}

impl From<usize> for GridSpan {
    fn from(val: usize) -> Self {
        GridSpan { val: Some(val) }
    }
}

__xml_test_suites!(
    GridSpan,
    GridSpan::default(),
    "<w:gridSpan/>",
    GridSpan::from(2),
    r#"<w:gridSpan w:val="2"/>"#,
    GridSpan::from(4),
    r#"<w:gridSpan w:val="4"/>"#,
);
