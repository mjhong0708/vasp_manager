use super::submit_slurm;
use crate::config_parser;
use clap::Parser;
use eyre::Result;

#[derive(Parser)]
pub struct CreateJob {
    /// The directory to create the job in.
    #[clap(short, long, default_value = ".")]
    pub dir: String,
    /// Whether submit job to slurm or not.
    #[clap(short, long, action)]
    pub submit: bool,
}

impl CreateJob {
    pub fn run(&self) -> Result<()> {
        let job_config = config_parser::JobConfig::from_dir(&self.dir)?;
        job_config.create_job()?;

        if self.submit {
            let job_name = &job_config.toml_contents["slurm"]["job_name"].as_str().unwrap();
            std::env::set_current_dir(job_config.job_dir)?;
            submit_slurm(job_name)?;
        }
        Ok(())
    }
}
