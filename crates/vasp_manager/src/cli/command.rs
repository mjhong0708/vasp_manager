use super::subcommands::*;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author = "Minjoon Hong <mjhong0708@yonsei.ac.kr>", version, about = "Vasp job manager", long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Initialize a new job directory with preconfigured Vasp.toml.
    /// See `vasp_manger init --help` for more information.
    #[clap(name = "init")]
    Init(InitJob),
    /// Create a job from Vasp.toml.
    #[clap(name = "create_job")]
    Create(CreateJob),
    /// Submit a job to slurm.
    #[clap(name = "submit_job")]
    Submit(SubmitJob),
    /// Continue existing job in new directory.
    #[clap(name = "continue_job")]
    Continue(ContinueJob),
    /// Show available templates for the INCAR file and their contents.
    #[clap(name = "show_incar")]
    ShowIncar(ShowIncar),
}
