pub mod check_util;
pub mod cli;
pub mod config_parser;
pub mod template;

use check_util::{check_task, check_vasp_bin, check_vasp_version};
use clap::Parser;
use std::process;
use std::{error::Error, path::Path};
use tera::Context;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Cli::parse();
    match args.command {
        cli::Command::Create(create_job_cfg) => {
            let job_config = config_parser::JobConfig::from_dir(&create_job_cfg.dir)?;
            job_config.create_job()?;

            if create_job_cfg.submit {
                let job_name = &job_config.toml_contents["slurm"]["job_name"].as_str().unwrap();
                std::env::set_current_dir(job_config.job_dir)?;
                let output = process::Command::new("sbatch")
                    .arg("-J")
                    .arg(job_name)
                    .arg("job_script.sh")
                    .output()?;
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
        }
        cli::Command::Init(init_job_cfg) => {
            // Render Vasp.toml template
            let mut context = Context::new();

            let vasp_version = &format!("\"{}\"", &check_vasp_version(&init_job_cfg.vasp_version)?);
            context.insert("vasp_version", vasp_version);

            let vasp_bin = &format!(
                "\"{}\"",
                &check_vasp_bin(&init_job_cfg.vasp_version, &init_job_cfg.vasp_bin)?
            );
            context.insert("vasp_bin", vasp_bin);

            let incar_base = &format!("\"{}\"", &check_task(&init_job_cfg.task)?);
            context.insert("incar_base", incar_base);

            let rendered = template::TERA.render("Vasp.toml", &context)?;

            // create job dir if it doesn't exist
            let job_dir = Path::new(&init_job_cfg.dir);
            if !job_dir.exists() {
                std::fs::create_dir_all(job_dir)?;
            }

            let config_file = job_dir.join("Vasp.toml");
            std::fs::write(config_file, rendered)?;
            println!("Created Vasp.toml in {}", &init_job_cfg.dir);
        }
        cli::Command::ShowIncar(show_incar) => {
            show_incar.show_incar_template();
        }
    }
    Ok(())
}
