//! Relationship item
//!
//! The corresponding ZIP item is `/_rels/.rels` (package-relationship) or
//! `/word/_rels/document.xml.rels` (part-relationship).

use hard_xml::{XmlRead, XmlResult, XmlWrite, XmlWriter};
use std::borrow::Cow;
use std::io::Write;

use crate::__string_enum;
use crate::schema::SCHEMA_RELATIONSHIPS;

#[derive(Debug, Default, XmlRead, Clone)]
#[xml(tag = "Relationships")]
pub struct Relationships<'a> {
    #[xml(child = "Relationship")]
    pub relationships: Vec<Relationship<'a>>,
}

impl<'a> XmlWrite for Relationships<'a> {
    fn to_writer<W: Write>(&self, writer: &mut XmlWriter<W>) -> XmlResult<()> {
        let Relationships { relationships } = self;

        log::debug!("[Relationships] Started writing.");
        let _ = write!(writer.inner, "{}", crate::schema::SCHEMA_XML);

        writer.write_element_start("Relationships")?;

        writer.write_attribute("xmlns", SCHEMA_RELATIONSHIPS)?;

        if relationships.is_empty() {
            writer.write_element_end_empty()?;
        } else {
            writer.write_element_end_open()?;
            for ele in relationships {
                ele.to_writer(writer)?;
            }
            writer.write_element_end_close("Relationships")?;
        }

        log::debug!("[Relationships] Finished writing.");

        Ok(())
    }
}

impl<'a> Relationships<'a> {
    pub fn add_rel(&mut self, schema: &'a str, target: &'a str) {
        let has = self.relationships.iter().find(|r| r.target == target);
        if has.is_none() {
            let ids: Vec<_> = self
                .relationships
                .iter()
                .map(|r| r.id.to_string())
                .collect();

            let len = self.relationships.len();

            let mut available = false;
            let mut id = len;
            while !available {
                id += 1;
                let idstr = format!("rId{}", id);
                available = !ids.contains(&idstr);
            }

            //hack
            //let target = target.replace("jpeg","png");
            self.relationships.push(Relationship {
                id: format!("rId{}", id).into(),
                target: target.into(),
                ty: schema.into(),
                target_mode: None,
            });
        }
    }

    /// Add a relationship and return the assigned ID
    pub fn add_rel_returning_id(&mut self, schema: &'a str, target: &'a str) -> String {
        // Check if relationship already exists
        if let Some(existing) = self.relationships.iter().find(|r| r.target == target) {
            return existing.id.to_string();
        }

        let ids: Vec<_> = self
            .relationships
            .iter()
            .map(|r| r.id.to_string())
            .collect();

        let len = self.relationships.len();

        let mut available = false;
        let mut id = len;
        while !available {
            id += 1;
            let idstr = format!("rId{}", id);
            available = !ids.contains(&idstr);
        }

        let id_str = format!("rId{}", id);
        self.relationships.push(Relationship {
            id: id_str.clone().into(),
            target: target.into(),
            ty: schema.into(),
            target_mode: None,
        });
        id_str
    }

    pub fn add_rel_with_target_mode(
        &mut self,
        schema: &'a str,
        target: &'a str,
        target_mode: Option<&'a str>,
    ) {
        let has = self.relationships.iter().find(|r| r.target == target);
        if has.is_none() {
            let ids: Vec<_> = self
                .relationships
                .iter()
                .map(|r| r.id.to_string())
                .collect();

            let len = self.relationships.len();

            let mut available = false;
            let mut id = len;
            while !available {
                id += 1;
                let idstr = format!("rId{}", id);
                available = !ids.contains(&idstr);
            }

            //hack
            //let target = target.replace("jpeg","png");
            self.relationships.push(Relationship {
                id: format!("rId{}", id).into(),
                target: target.into(),
                ty: schema.into(),
                target_mode: TargetMode::from_str(target_mode),
            });
        }
    }

    pub fn get_target(&self, id: &str) -> Option<&str> {
        self.relationships
            .iter()
            .find(|r| r.id == id)
            .map(|r| &*r.target)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(Eq))]
pub enum TargetMode {
    Internal,
    External,
}

__string_enum! {
    TargetMode {
        Internal = "Internal",
        External = "External",
    }
}

impl From<&str> for TargetMode {
    fn from(value: &str) -> Self {
        match value {
            "External" => TargetMode::External,
            _ => TargetMode::Internal,
        }
    }
}

impl TargetMode {
    fn from_str(option_str: Option<&str>) -> Option<Self> {
        match option_str {
            Some(s) => Some(s.into()),
            None => None,
        }
    }
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[xml(tag = "Relationship")]
pub struct Relationship<'a> {
    #[xml(attr = "Id")]
    pub id: Cow<'a, str>,
    #[xml(attr = "Target")]
    pub target: Cow<'a, str>,
    #[xml(attr = "Type")]
    pub ty: Cow<'a, str>,
    #[xml(attr = "TargetMode")]
    pub target_mode: Option<TargetMode>,
}
