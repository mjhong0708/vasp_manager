use eyre::Result;
use std::fs::read_to_string;
use std::path::Path;
use toml::Value;

pub fn format_value(value: &Value) -> String {
    let formatted = format!("{}", value);
    if formatted.starts_with('"') && formatted.ends_with('"') {
        formatted[1..formatted.len() - 1].to_string()
    } else {
        formatted
    }
}

/// Job configuration.
/// Implements VASP job related methods.
pub struct JobConfig {
    pub toml_contents: Value,
    pub job_dir: String,
}

impl JobConfig {
    /// Parses the config file and returns a `JobConfig` object.
    pub fn from_dir(job_dir: &str) -> Result<JobConfig> {
        let config_file = Path::new(job_dir).join("Vasp.toml");
        let toml_str = read_to_string(config_file).map_err(|_| eyre::eyre!("Vasp.toml not found."))?;
        let toml_contents = toml::from_str(&toml_str).map_err(|_| eyre::eyre!("Could not parse Vasp.toml."))?;
        Ok(JobConfig {
            toml_contents,
            job_dir: job_dir.into(),
        })
    }

    /// Writes input files needed for VASP and SLURM.
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
}
