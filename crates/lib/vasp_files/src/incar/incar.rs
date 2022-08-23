use std::{collections::BTreeMap, str::FromStr};

use super::data::INCAR_TAGS;
use super::incar_tag::IncarTag;

#[derive(Debug)]
pub struct Incar {
    pub tags: BTreeMap<String, IncarTag>,
}

impl Incar {
    pub fn update(&mut self, tag: IncarTag) -> Result<(), String> {
        let name = tag.name();
        if INCAR_TAGS.contains_key(name) {
            self.tags.insert(name.into(), tag);
            Ok(())
        } else {
            Err(format!("Invalid tag name: {}", name))
        }
    }

    pub fn extend(&mut self, tags: Vec<IncarTag>) -> Result<(), String> {
        for tag in tags {
            self.update(tag)?;
        }
        Ok(())
    }
}

impl From<Vec<IncarTag>> for Incar {
    fn from(tags: Vec<IncarTag>) -> Self {
        let mut incar = Incar { tags: BTreeMap::new() };
        for tag in tags {
            incar.update(tag).unwrap();
        }
        incar
    }
}

impl FromStr for Incar {
    type Err = String;
    /// Parse INCAR file from string.
    /// Ignores malformed lines, as VASP does.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tags = {
            let tags: Vec<IncarTag> = s.lines().filter_map(|line| line.parse::<IncarTag>().ok()).collect();
            let names: Vec<String> = tags.iter().map(|tag| tag.name().to_string()).collect();
            BTreeMap::from_iter(names.into_iter().zip(tags.into_iter()))
        };

        Ok(Incar { tags })
    }
}

impl ToString for Incar {
    fn to_string(&self) -> String {
        self.tags
            .values()
            .map(|tag| tag.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
