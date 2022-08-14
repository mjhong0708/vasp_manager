use super::template::{IncarTag, TERA};
use kpoints::{Kpoints, KpointsScheme};
use poscar::Poscar;
use potcar::{generate_potcar, PotcarMode};

use eyre::Result;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;
use tera::Context;
use toml::Value;

pub fn format_value(value: &Value) -> String {
    let formatted = format!("{}", value);
    if formatted.starts_with('"') && formatted.ends_with('"') {
        formatted[1..formatted.len() - 1].to_string()
    } else {
        formatted
    }
}

pub struct JobConfig {
    pub toml_contents: Value,
    pub job_dir: String,
}

impl JobConfig {
    pub fn from_dir(job_dir: &str) -> Result<JobConfig> {
        let config_file = Path::new(job_dir).join("Vasp.toml");
        let toml_str = read_to_string(config_file).map_err(|_| eyre::eyre!("Vasp.toml not found."))?;
        let toml_contents = toml::from_str(&toml_str).map_err(|_| eyre::eyre!("Could not parse Vasp.toml."))?;
        Ok(JobConfig {
            toml_contents,
            job_dir: job_dir.into(),
        })
    }

    pub fn create_job(&self) -> Result<()> {
        println!("Creating job in {}", self.job_dir);
        println!("Writing POTCAR");
        self.write_potcar()?;
        println!("Writing KPOINTS");
        self.write_kpoints()?;
        println!("Writing INCAR");
        self.write_incar()?;
        println!("Writing Job script");
        self.write_job_script()?;
        Ok(())
    }

    fn write_potcar(&self) -> Result<()> {
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

    fn write_kpoints(&self) -> Result<()> {
        let kpoints_config = &self.toml_contents["vasp"]["kpoints"];
        let scheme: KpointsScheme = kpoints_config["scheme"]
            .as_str()
            .unwrap()
            .parse()
            .map_err(|_| eyre::eyre!("Unrecognized Kpoints scheme"))?;

        let kpoints = match kpoints_config.get("mesh") {
            Some(m) => {
                let mesh = [
                    m[0].as_integer().unwrap() as u32,
                    m[1].as_integer().unwrap() as u32,
                    m[2].as_integer().unwrap() as u32,
                ];
                Kpoints::new(scheme, mesh)
            }
            None => {
                let density = kpoints_config["density"]
                    .as_float()
                    .expect("Please specify mesh or k-points density");
                let lattice = Poscar::from_file("POSCAR").lattice;
                Kpoints::from_density(scheme, density, lattice)
            }
        };

        let kpoints_str = kpoints.to_string();
        let kpoints_path = format!("{}/KPOINTS", self.job_dir);
        std::fs::write(kpoints_path, kpoints_str)?;
        Ok(())
    }

    fn write_incar(&self) -> Result<()> {
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

    fn write_job_script(&self) -> Result<()> {
        let vasp_config = &self.toml_contents["vasp"];
        let slurm_config = &self.toml_contents["slurm"];

        let mut context = Context::new();
        context.insert("partition", &format_value(&slurm_config["partition"]));
        context.insert("num_nodes", &format_value(&slurm_config["num_nodes"]));
        context.insert("num_tasks", &format_value(&slurm_config["num_tasks"]));
        context.insert("vasp_version", &format_value(&vasp_config["version"]));
        context.insert("bin", &format_value(&vasp_config["bin"]));

        let rendered = TERA.render("job_script.sh", &context).unwrap();
        let job_script_path = format!("{}/job_script.sh", self.job_dir);
        std::fs::write(job_script_path, rendered)?;
        Ok(())
    }
}
