use clap::Parser;
use eyre::Result;
use std::ffi::OsStr;
use std::path::Path;

#[derive(Parser)]
pub struct ContinueJob {
    /// The directory to continue the job in.
    #[clap(short, long)]
    pub dir: String,
}

impl ContinueJob {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // create job dir if it doesn't exist
        // If already exists, exit.
        let job_dir = Path::new(&self.dir);
        if !job_dir.exists() {
            std::fs::create_dir_all(job_dir)?;
        } else {
            eprintln!("{} already exists. Aborting...", &self.dir);
            std::process::exit(1);
        }

        // If CONTCAR does not exist, exit.
        if !Path::new("CONTCAR").exists() {
            eprintln!("CONTCAR does not exist. Aborting...");
            std::process::exit(1);
        }

        copy_if_nonempty("CONTCAR", &job_dir.join("POSCAR"))?;
        let files_to_copy = vec!["INCAR", "POTCAR", "KPOINTS", "job_script.sh", "WAVECAR", "CHGCAR"];
        for file in files_to_copy {
            println!("Copying {}", file);
            copy_if_nonempty(file, &job_dir.join(file))?;
        }

        println!("Created continue job in {}", &self.dir);
        Ok(())
    }
}

fn copy_if_nonempty<T, S>(file: T, dest: S) -> Result<()>
where
    T: AsRef<Path> + AsRef<OsStr>,
    S: AsRef<Path> + AsRef<OsStr>,
{
    let src = Path::new(&file);
    let dest = Path::new(&dest);
    // check if src is not empty
    let file_size = std::fs::metadata(src)?.len();
    if src.exists() && file_size > 0 {
        std::fs::copy(src, dest)?;
    }
    Ok(())
}
