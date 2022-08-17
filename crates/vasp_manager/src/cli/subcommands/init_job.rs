use crate::check_util::*;
use crate::template;
use clap::Parser;
use eyre::Result;
use std::path::Path;
use tera::Context;

#[derive(Parser)]
pub struct InitJob {
    /// The directory to create the job in. If does not exist, it will be created.
    #[clap(short, long, default_value = ".")]
    pub dir: String,
    /// The version of VASP.
    #[clap(short, long, default_value = "6.3.1")]
    pub vasp_version: String,
    /// VASP binary to use. 'vasp_std', 'vasp_gam', ...
    #[clap(long = "bin", default_value = "vasp_std")]
    pub vasp_bin: String,
    /// Task of the job, which is suffix of INCAR template.
    /// To see all available tasks, run `vasp_manager show_incar`.
    /// Default is 'relax'.
    #[clap(long, default_value = "relax")]
    pub task: String,
}

impl InitJob {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Render Vasp.toml template
        let vasp_toml_rendered = {
            let mut context = Context::new();

            let vasp_version = &format!("\"{}\"", &check_vasp_version(&self.vasp_version)?);
            context.insert("vasp_version", vasp_version);

            let vasp_bin = &format!("\"{}\"", &check_vasp_bin(&self.vasp_version, &self.vasp_bin)?);
            context.insert("vasp_bin", vasp_bin);

            let incar_base = &format!("\"{}\"", &check_task(&self.task)?);
            context.insert("incar_base", incar_base);

            template::TEMPLATES.render("Vasp.toml", &context)?
        };

        let job_dir = Path::new(&self.dir);
        // create job dir if it doesn't exist
        if !job_dir.exists() {
            std::fs::create_dir_all(job_dir)?;
        }
        // write Vasp.toml
        let config_file = job_dir.join("Vasp.toml");
        std::fs::write(config_file, vasp_toml_rendered)?;
        println!("Created Vasp.toml in {}", &self.dir);
        Ok(())
    }
}
