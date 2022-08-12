use lazy_static::lazy_static;
use serde::Serialize;
pub use tera::Tera;

pub static JOB_SCRIPT: &str = include_str!("templates/job_script.sh");
pub static INCAR_RELAX: &str = include_str!("templates/INCAR_relax");
pub static INCAR_SINGLEPOINT: &str = include_str!("templates/INCAR_singlepoint");
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
        tera.add_raw_template("INCAR_relax", INCAR_RELAX).unwrap();
        tera.add_raw_template("INCAR_singlepoint", INCAR_SINGLEPOINT).unwrap();
        tera.add_raw_template("KPOINTS", KPOINTS).unwrap();
        tera.add_raw_template("Vasp.toml", VASP_TOML).unwrap();
        tera
    };
}
