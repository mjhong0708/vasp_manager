use super::submit_slurm;
use crate::config_parser;
use clap::Parser;
use eyre::Result;
use inquire::Confirm;

#[derive(Parser)]
pub struct SubmitJob {
    /// If set, command does not inquire the user for reviewing the input.
    #[clap(short = 'r', long, action)]
    pub no_review: bool,
}

impl SubmitJob {
    pub fn run(&self) -> Result<()> {
        let job_config = config_parser::JobConfig::from_dir(".")?;
        let job_name = job_config.toml_contents["slurm"]["job_name"].as_str().unwrap();

        if !self.no_review {
            let config_file = std::fs::read_to_string("Vasp.toml")?;
            println!("======= Vasp.toml =======");
            println!("{}", config_file);
            println!("=========================");
            let ans = Confirm::new("Are you sure you want to submit this job?")
                .with_default(false)
                .with_help_message("Please review your settings.")
                .prompt();

            match ans {
                Ok(true) => {
                    submit_slurm(job_name)?;
                    std::process::exit(0);
                }
                Ok(false) => {
                    println!("Job submission aborted.");
                    std::process::exit(1);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }

        submit_slurm(job_name)?;
        Ok(())
    }
}
