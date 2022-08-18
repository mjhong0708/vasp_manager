pub use clap::Parser;
pub mod continue_job;
pub mod create_job;
pub mod init_job;
pub mod show_incar;
pub mod submit_job;

pub use continue_job::*;
pub use create_job::*;
pub use init_job::*;
pub use show_incar::*;
pub use submit_job::*;

use eyre::Result;

/// Submits slurm job in current directory.
pub fn submit_slurm(job_name: &str) -> Result<()> {
    let output = std::process::Command::new("sbatch")
        .arg("-J")
        .arg(&format!("'{}'", job_name))
        .arg("job_script.sh")
        .output()?;
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}
