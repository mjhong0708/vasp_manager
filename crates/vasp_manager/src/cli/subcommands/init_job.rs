use crate::check_util::*;
use crate::template;
use clap::Parser;
use eyre::Result;
use std::path::Path;
use tera::Context;

#[derive(Parser)]
pub struct InitJob {
    /// The directory to create the job in.
    #[clap(short, long, default_value = ".")]
    pub dir: String,
    /// The version of VASP
    #[clap(short, long, default_value = "6.3.1")]
    pub vasp_version: String,
    /// VASP binary to use. 'vasp_std', 'vasp_gam', ...
    #[clap(long = "bin", default_value = "vasp_std")]
    pub vasp_bin: String,
    /// Task of the job. Default is 'relax'. `singlepoint` is also available.
    #[clap(long, default_value = "relax")]
    pub task: String,
}

impl InitJob {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Render Vasp.toml template
        let mut context = Context::new();

        let vasp_version = &format!("\"{}\"", &check_vasp_version(&self.vasp_version)?);
        context.insert("vasp_version", vasp_version);

        let vasp_bin = &format!("\"{}\"", &check_vasp_bin(&self.vasp_version, &self.vasp_bin)?);
        context.insert("vasp_bin", vasp_bin);

        let incar_base = &format!("\"{}\"", &check_task(&self.task)?);
        context.insert("incar_base", incar_base);

        let rendered = template::TERA.render("Vasp.toml", &context)?;

        // create job dir if it doesn't exist
        let job_dir = Path::new(&self.dir);
        if !job_dir.exists() {
            std::fs::create_dir_all(job_dir)?;
        }

        let config_file = job_dir.join("Vasp.toml");
        std::fs::write(config_file, rendered)?;
        println!("Created Vasp.toml in {}", &self.dir);
        Ok(())
    }
}
