use super::config;
use super::config::format_value;
use crate::template::{IncarTag, TERA};
use eyre::Result;
use tera::Context;

impl config::JobConfig {
    pub fn write_incar(&self) -> Result<()> {
        let incar_config = &self.toml_contents["vasp"]["incar"];
        let base_incar = {
            let base = incar_config["base"].as_str().unwrap();
            format!("INCAR_{}", base)
        };
        let additional_tags: Vec<IncarTag> = match incar_config.get("tags") {
            Some(tags) => tags
                .as_table()
                .unwrap()
                .iter()
                .map(|(name, value)| IncarTag {
                    name: name.to_string(),
                    value: format_value(value),
                })
                .collect(),
            None => vec![],
        };
        let mut context = Context::new();
        context.insert("incar_tags", &additional_tags);
        let rendered = TERA
            .render(&base_incar, &context)
            .unwrap_or_else(|_| panic!("Template {} not found.", base_incar));
        let incar_path = format!("{}/INCAR", self.job_dir);
        std::fs::write(incar_path, rendered)?;
        Ok(())
    }
}
