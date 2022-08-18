use crate::template;
use clap::Parser;
use eyre::Result;
use inquire::error::InquireError;
use inquire::Select;
use std::collections::HashMap;

#[derive(Parser)]
pub struct ShowIncar;

impl ShowIncar {
    pub fn run(&self) -> Result<()> {
        let mut all_incar_templates = vec![];
        let mut all_incar_descriptions = vec![];
        for key in template::INCAR_TEMPLATES.keys() {
            let description = format!("{}  [{}]", key, template::INCAR_TEMPLATES[key].1);
            all_incar_templates.push(key.to_string());
            all_incar_descriptions.push(description.to_string());
        }
        let mapping: HashMap<&String, &String> =
            HashMap::from_iter(all_incar_descriptions.iter().zip(all_incar_templates.iter()));

        let selected_incar: Result<String, InquireError> =
            Select::new("Choose INCAR template to see.", all_incar_descriptions.clone()).prompt();
        match selected_incar {
            Ok(desc) => {
                let key = mapping[&desc];
                println!("========= {} =========", key);
                let (template, _) = template::INCAR_TEMPLATES[key];
                println!("{}", template);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
        Ok(())
    }
}
