pub mod cli;
pub mod config_parser;
pub mod potcargen;
pub mod template;
pub mod util;

use clap::Parser;
use std::process;
use std::{error::Error, path::Path};
use tera::Context;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Cli::parse();
    match args.command {
        cli::Command::Create(create_job) => {
            let job_config = config_parser::JobConfig::from_dir(&create_job.dir)?;
            job_config.create_job()?;
            if create_job.submit {
                println!("Submitting job");
                std::env::set_current_dir(job_config.job_dir)?;
                let output = process::Command::new("sbatch").arg("job_script.sh").output()?;
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
        }
        cli::Command::Init(init_job) => {
            let mut context = Context::new();
            context.insert("vasp_version", &format!("\"{}\"", &init_job.vasp_version));
            context.insert("vasp_bin", &format!("\"{}\"", &init_job.vasp_bin));
            context.insert("incar_base", &format!("\"{}\"", &init_job.task));
            let rendered = template::TERA.render("Vasp.toml", &context)?;
            let job_dir = Path::new(&init_job.dir);
            // create job dir if it doesn't exist
            if !job_dir.exists() {
                std::fs::create_dir_all(job_dir)?;
            }
            let config_file = job_dir.join("Vasp.toml");
            std::fs::write(config_file, rendered)?;
            println!("Created Vasp.toml in {}", &init_job.dir);
        }
    }
    Ok(())
}
