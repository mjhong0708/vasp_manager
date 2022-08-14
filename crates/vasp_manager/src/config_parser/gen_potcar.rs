use super::config;
use eyre::Result;
use potcar::{generate_potcar, PotcarMode};
use std::collections::HashMap;

impl config::JobConfig {
    pub fn write_potcar(&self) -> Result<()> {
        let potcar_mode = match &self.toml_contents["vasp"]["potcar"].as_str() {
            Some(s) if s == &"recommended" => PotcarMode::Recommended,
            Some(s) => panic!("Unknown potcar mode: {}", s),
            None => {
                let potcar_map = {
                    let map = &self.toml_contents["potcar"].as_table().expect("Unrecognized format");
                    let mut potcar_map: HashMap<String, String> = HashMap::new();
                    for (k, v) in map.iter() {
                        potcar_map.insert(k.clone(), v.as_str().unwrap().to_string());
                    }
                    potcar_map
                };
                PotcarMode::Custom(potcar_map)
            }
        };
        let input_file = &self.toml_contents["vasp"]["input"].as_str().unwrap();
        generate_potcar(&self.job_dir, input_file, potcar_mode)
    }
}
