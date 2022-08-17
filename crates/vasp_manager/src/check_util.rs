use super::template;
use glob::glob;
use home;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::env;

lazy_static! {
    pub static ref VASP_DIR: String = {
        let vasp_dir = match env::var("VASP_DIR") {
            Ok(vasp_dir) => vasp_dir,
            Err(_) => {
                let vasp_dir = home::home_dir().unwrap().join(".local").join("VASP").join("bin");
                vasp_dir.to_str().unwrap().to_string()
            }
        };
        vasp_dir
    };
    pub static ref VASP_VERSIONS: Vec<String> = {
        glob(&format!("{}/*.*.*", &*VASP_DIR))
            .unwrap()
            .map(|path| path.unwrap().to_str().unwrap().to_string())
            .map(|path| path.split('/').last().unwrap().to_string())
            .collect()
    };
    pub static ref VASP_BINS: HashMap<String, Vec<String>> = {
        let mut vasp_bins: HashMap<String, Vec<String>> = HashMap::new();
        for version in &*VASP_VERSIONS {
            let version_dir = format!("{}/{}", &*VASP_DIR, version);
            let all_bins = glob(&format!("{}/*", version_dir))
                .unwrap()
                .map(|path| path.unwrap().file_name().unwrap().to_str().unwrap().to_string())
                .collect::<Vec<String>>();
            vasp_bins.insert(version.clone(), all_bins);
        }
        vasp_bins
    };
}

/// Checks if the vasp version is available.
pub fn check_vasp_version(version: &str) -> Result<String, String> {
    match VASP_VERSIONS.contains(&version.to_string()) {
        true => Ok(version.to_string()),
        false => Err(format!(
            "Vasp version {} not found. Currently available versions: {}",
            version,
            VASP_VERSIONS.join(", ")
        )),
    }
}

/// Checks if the task is available in INCAR templates.
pub fn check_task(task: &str) -> Result<String, String> {
    let available_tasks: Vec<&str> = template::TEMPLATES
        .get_template_names()
        .filter_map(|name| match name.contains("INCAR") {
            true => Some(name.split('_').last().unwrap()),
            false => None,
        })
        .collect();

    match available_tasks.iter().any(|name| name == &task) {
        true => Ok(task.to_string()),
        false => Err(format!(
            "Task '{}' not found. Available tasks: {}",
            task,
            available_tasks.join(", ")
        )),
    }
}

/// Checks if the vasp binary is available.
pub fn check_vasp_bin(version: &str, task: &str) -> Result<String, String> {
    match VASP_BINS.get(version) {
        Some(bins) => match bins.iter().any(|bin| bin == task) {
            true => Ok(task.to_string()),
            false => Err(format!(
                "Vasp binary '{}' not found. Currently available binaries for VASP {}:\n{}",
                task,
                version,
                bins.join(", ")
            )),
        },
        None => Err(format!(
            "Vasp version {} not found. Currently available versions: \n{}",
            version,
            VASP_VERSIONS.join(", ")
        )),
    }
}

mod tests {
    #[test]
    fn test_vasp_versions() {
        use super::*;
        for version in VASP_VERSIONS.iter() {
            println!("{}", version);
        }
        assert!(VASP_VERSIONS.len() > 0);
    }
}
