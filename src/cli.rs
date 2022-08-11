use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[clap(name = "create_job")]
    Create(CreateJob),
    Init(InitJob),
}

#[derive(Parser)]
pub struct CreateJob {
    /// The directory to create the job in.
    #[clap(short, long, default_value = ".")]
    pub dir: String,
    /// Whether submit job to slurm or not.
    #[clap(short, long, action)]
    pub submit: bool,
}

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
