pub mod check_util;
pub mod cli;

use clap::Parser;
use home;
use std::error::Error;
use std::path::Path;
use vasp_manager::{config_parser, template};

fn check_setup_status() -> Result<(), String> {
    let home_dir = home::home_dir().unwrap();
    // Check vasp binary
    let vasp_dir = home_dir.join(".local/VASP/bin");
    if !vasp_dir.exists() {
        return Err(String::from("VASP binaries are not set up."));
    }
    let potcar_path_prefix = std::env::var("POTCAR_PATH_PREFIX");
    if potcar_path_prefix.is_err() {
        return Err(String::from("POTCAR_PATH_PREFIX is not set up."));
    } else {
        let potcar_path = potcar_path_prefix.unwrap();
        let potcar_path = Path::new(&potcar_path);
        if !&potcar_path.exists() || !&potcar_path.is_dir() || &potcar_path.read_dir().unwrap().count() == &0 {
            return Err(String::from("POTCAR_PATH is not properly set up."));
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Cli::parse();

    if args.show_guide {
        vasp_manager::cli::print_guide();
        std::process::exit(0);
    }

    match check_setup_status() {
        Ok(_) => (),
        Err(e) => {
            println!("Setup error: {}", e);
            println!("It seems that setup is not done");
            println!("Run `vasp_manager --guide ` to see hot to set up VASP binaries and POTCAR_PATH.");
            std::process::exit(1);
        }
    }

    match &args.command {
        Some(cli::Command::Init(init_job)) => init_job.run()?,
        Some(cli::Command::Create(create_job)) => create_job.run()?,
        Some(cli::Command::Submit(submit_job)) => submit_job.run()?,
        Some(cli::Command::Continue(continue_job)) => continue_job.run()?,
        Some(cli::Command::ShowIncar(show_incar)) => show_incar.run()?,
        None => (),
    }
    Ok(())
}
