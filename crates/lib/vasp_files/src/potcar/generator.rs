use super::core::{PotcarMode, POTCAR_DATA};
use super::util::extract_elements;
use eyre::Result;
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::{env, fs};

fn get_recommended_potcars(elems: &[String]) -> Vec<String> {
    elems
        .iter()
        .map(|e| POTCAR_DATA.iter().find(|p| &p.element == e && p.recommended).unwrap())
        .map(|p| p.potcar_name.to_string())
        .collect()
}

fn get_potcars_from_map(elems: &[String], potcar_map: &HashMap<String, String>) -> Vec<String> {
    elems.iter().map(|e| potcar_map.get(e).unwrap().to_string()).collect()
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
    let elements = extract_elements(&input_file_path);
    let potcars = match mode {
        PotcarMode::Recommended => get_recommended_potcars(&elements),
        PotcarMode::Custom(potcar_map) => get_potcars_from_map(&elements, &potcar_map),
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
