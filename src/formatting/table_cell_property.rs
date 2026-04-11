use hard_xml::{XmlRead, XmlWrite};

use crate::{__setter, __xml_test_suites};

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:tcPr")]
pub struct TableCellProperty<'a> {
    #[xml(child = "w:tcW")]
    pub wide: Option<super::TableCellWidth>,
    #[xml(child = "w:gridSpan")]
    pub grid_span: Option<super::GridSpan>,
    #[xml(child = "w:vMerge")]
    pub v_merge: Option<super::VMerge>,
    #[xml(child = "w:tcBorders")]
    pub t_c_borders: Option<super::TableCellBorders<'a>>,
    #[xml(child = "w:shd")]
    pub shading: Option<super::Shading<'a>>,
    #[xml(default, child = "w:vAlign")]
    pub v_align: super::VAlign,
}

impl<'a> TableCellProperty<'a> {
    __setter!(v_align: super::VAlign);
    __setter!(wide: Option<super::TableCellWidth>);
    __setter!(grid_span: Option<super::GridSpan>);
    __setter!(v_merge: Option<super::VMerge>);
    __setter!(t_c_borders: Option<super::TableCellBorders<'a>>);
    __setter!(shading: Option<super::Shading<'a>>);
}

__xml_test_suites!(
    TableCellProperty,
    TableCellProperty::default(),
    r#"<w:tcPr><w:vAlign w:val="top"/></w:tcPr>"#,
    TableCellProperty::default().v_align(super::VAlignType::Bottom),
    r#"<w:tcPr><w:vAlign w:val="bottom"/></w:tcPr>"#,
    TableCellProperty::default().shading(super::Shading {
        fill: Some(std::borrow::Cow::Borrowed("EAF2FB")),
        ..super::Shading::default()
    }),
    r#"<w:tcPr><w:shd w:fill="EAF2FB"/><w:vAlign w:val="top"/></w:tcPr>"#,
    TableCellProperty::default()
        .t_c_borders(super::TableCellBorders::default().top(super::TopBorder::default())),
    r#"<w:tcPr><w:tcBorders><w:top w:val="none"/></w:tcBorders><w:vAlign w:val="top"/></w:tcPr>"#,
);
