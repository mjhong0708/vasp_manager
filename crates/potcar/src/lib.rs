pub mod poscar;
pub mod potcar;
use eyre::Result;
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::{env, fs};

pub enum PotcarMode {
    Recommended,
    Custom(HashMap<String, String>),
}

pub fn generate_potcar(input_dir: &str, input_poscar: &str, mode: PotcarMode) -> Result<()> {
    let potcar_path = match env::var("POTCAR_PATH_PREFIX") {
        Ok(p) => p,
        Err(_) => {
            panic!("The environment variable POTCAR_PATH_PREFIX is not set.");
        }
    };
    let potcar_destination = Path::new(input_dir).join("POTCAR");

    let input_file_path = format!("{}/{}", input_dir, input_poscar);
    let elements = poscar::extract_elements(&input_file_path);
    let potcars = match mode {
        PotcarMode::Recommended => potcar::get_recommended_potcars(&elements),
        PotcarMode::Custom(potcar_map) => potcar::get_potcars_from_map(&elements, &potcar_map),
    };
    let potcar_paths = potcars
        .into_iter()
        .map(|p| potcar_path.to_string() + "/" + &p + "/POTCAR");
    if Path::new(&potcar_destination).exists() {
        fs::remove_file(&potcar_destination).expect("Failed to delete POTCAR file.");
    }

    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&potcar_destination)?;

    potcar_paths.for_each(|path| {
        let potcar_str = fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read POTCAR file {}", &path));
        write!(file, "{}", potcar_str).expect("Failed to write to POTCAR file.");
    });
    Ok(())
}
