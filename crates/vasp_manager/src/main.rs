pub mod check_util;
pub mod cli;

use clap::Parser;
use std::error::Error;

use vasp_manager::{config_parser, template};

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Cli::parse();
    match args.command {
        cli::Command::Init(init_job) => init_job.run()?,
        cli::Command::Create(create_job) => create_job.run()?,
        cli::Command::Continue(continue_job) => continue_job.run()?,
        cli::Command::ShowIncar(show_incar) => show_incar.run(),
    }
    Ok(())
}
