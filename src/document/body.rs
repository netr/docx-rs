use derive_more::From;
use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Borrow;

use crate::__xml_test_suites;
use crate::document::{Drawing, Paragraph, ParagraphContent, Run, RunContent, Table, TableCell};
use crate::formatting::SectionProperty;

use super::SDT;

/// Document Body
///
/// This is the main document editing surface.
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:body")]
pub struct Body<'a> {
    /// Specifies the contents of the body of the document.
    #[xml(child = "w:p", child = "w:tbl", child = "w:sectPr", child = "w:sdt")]
    pub content: Vec<BodyContent<'a>>,
}

impl<'a> Body<'a> {
    pub fn push<T: Into<BodyContent<'a>>>(&mut self, content: T) -> &mut Self {
        self.content.push(content.into());
        self
    }

    pub fn text(&self) -> String {
        let mut v: Vec<String> = Vec::new();
        for content in &self.content {
            match content {
                BodyContent::Paragraph(para) => {
                    v.push(para.text());
                    v.extend(extract_textbox_text_from_paragraph(para));
                }
                BodyContent::Table(_) => {}
                BodyContent::SectionProperty(_) => {}
                BodyContent::Sdt(sdt) => v.push(sdt.text()),
                BodyContent::TableCell(_) => {}
                BodyContent::Run(_) => {}
            }
        }
        v.join("\r\n")
    }

    pub fn replace_text_simple<S>(&mut self, old: S, new: S)
    where
        S: AsRef<str>,
    {
        let _d = self.replace_text(&[(old, new)]);
    }

    pub fn replace_text<'b, I, T, S>(&mut self, dic: T) -> crate::DocxResult<()>
    where
        S: AsRef<str> + 'b,
        T: IntoIterator<Item = I> + Copy,
        I: Borrow<(S, S)>,
    {
        for content in self.content.iter_mut() {
            match content {
                BodyContent::Paragraph(p) => {
                    p.replace_text(dic)?;
                }
                BodyContent::Table(t) => {
                    t.replace_text(dic)?;
                }
                BodyContent::SectionProperty(_) => {}
                BodyContent::Sdt(_) => {}
                BodyContent::TableCell(_) => {}
                BodyContent::Run(_) => {}
            }
        }
        Ok(())
    }

    // pub fn iter_text(&self) -> impl Iterator<Item = &Cow<'a, str>> {
    //     self.content
    //         .iter()
    //         .filter_map(|content| match content {
    //             BodyContent::Paragraph(para) => Some(para.iter_text()),
    //         })
    //         .flatten()
    // }

    // pub fn iter_text_mut(&mut self) -> impl Iterator<Item = &mut Cow<'a, str>> {
    //     self.content
    //         .iter_mut()
    //         .filter_map(|content| match content {
    //             BodyContent::Paragraph(para) => Some(para.iter_text_mut()),
    //         })
    //         .flatten()
    // }
}

fn extract_textbox_text_from_paragraph<'a>(paragraph: &'a Paragraph<'a>) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    for para_content in &paragraph.content {
        if let ParagraphContent::Run(run) = para_content {
            for run_content in &run.content {
                match run_content {
                    RunContent::Drawing(drawing) => {
                        collect_text_from_drawing(drawing, &mut results);
                    }
                    RunContent::AlternateContent(ac) => {
                        for choice in &ac.choices {
                            for drawing in &choice.drawings {
                                collect_text_from_drawing(drawing, &mut results);
                            }
                        }
                        if let Some(fallback) = &ac.fallback {
                            for drawing in &fallback.drawings {
                                collect_text_from_drawing(drawing, &mut results);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    results
}

fn collect_text_from_drawing<'a>(drawing: &'a Drawing<'a>, out: &mut Vec<String>) {
    if let Some(inline) = &drawing.inline {
        if let Some(graphic) = &inline.graphic {
            for wsp in &graphic.data.wps {
                if let Some(txbx) = &wsp.txbx {
                    if let Some(content) = &txbx.content {
                        for p in &content.paragraphs {
                            let t = p.text();
                            if !t.is_empty() {
                                out.push(t);
                            }
                        }
                    }
                }
            }
        }
    }
    if let Some(anchor) = &drawing.anchor {
        if let Some(graphic) = &anchor.graphic {
            for wsp in &graphic.data.wps {
                if let Some(txbx) = &wsp.txbx {
                    if let Some(content) = &txbx.content {
                        for p in &content.paragraphs {
                            let t = p.text();
                            if !t.is_empty() {
                                out.push(t);
                            }
                        }
                    }
                }
            }
        }
    }
}

/// A set of elements that can be contained in the body
#[derive(Debug, From, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum BodyContent<'a> {
    #[xml(tag = "w:p")]
    Paragraph(Paragraph<'a>),
    #[xml(tag = "w:tbl")]
    Table(Table<'a>),
    #[xml(tag = "w:sdt")]
    Sdt(SDT<'a>),
    #[xml(tag = "w:sectPr")]
    SectionProperty(SectionProperty<'a>),
    #[xml(tag = "w:tc")]
    TableCell(TableCell<'a>),
    #[xml(tag = "w:r")]
    Run(Run<'a>),
}

__xml_test_suites!(
    Body,
    Body::default(),
    r#"<w:body/>"#,
    Body {
        content: vec![Paragraph::default().into()]
    },
    r#"<w:body><w:p/></w:body>"#,
    Body {
        content: vec![Table::default().into()]
    },
    r#"<w:body><w:tbl><w:tblPr/><w:tblGrid/></w:tbl></w:body>"#,
);
