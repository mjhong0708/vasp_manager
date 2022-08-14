use lazy_static::lazy_static;
use phf::phf_map;
use serde::Serialize;
pub use tera::Tera;

pub static JOB_SCRIPT: &str = include_str!("templates/job_script.sh");
pub static INCAR_TEMPLATES: phf::Map<&'static str, (&'static str, &'static str)> = phf_map! {
    "INCAR_relax" => (include_str!("templates/INCAR_relax"), "Standard ionic relaxation"),
    "INCAR_singlepoint" => (include_str!("templates/INCAR_singlepoint"), "Single point calculation"),
    "INCAR_bader" => (include_str!("templates/INCAR_bader"), "Bader analysis"),
    "INCAR_aimd" => (include_str!("templates/INCAR_aimd"), "AIMD calculation (NVT)"),
};
pub static KPOINTS: &str = include_str!("templates/KPOINTS");
pub static VASP_TOML: &str = include_str!("templates/Vasp.toml.template");

#[derive(Serialize)]
pub struct IncarTag {
    pub name: String,
    pub value: String,
}

lazy_static! {
    pub static ref TERA: Tera = {
        let mut tera = Tera::default();
        tera.add_raw_template("job_script.sh", JOB_SCRIPT).unwrap();
        tera.add_raw_template("KPOINTS", KPOINTS).unwrap();
        tera.add_raw_template("Vasp.toml", VASP_TOML).unwrap();
        for name in INCAR_TEMPLATES.keys() {
            let (template, _) = INCAR_TEMPLATES[name];
            tera.add_raw_template(name, template).unwrap();
        }
        tera
    };
}
