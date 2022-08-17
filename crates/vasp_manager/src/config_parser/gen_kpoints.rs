use super::config;
use eyre::Result;
use kpoints::{Kpoints, KpointsScheme};
use poscar::Poscar;

impl config::JobConfig {
    /// Generates KPOINTS file from POSCAR.
    /// Available schemes are "Gamma" and "Monkhorst-Pack".
    /// User can either specify the mesh or density in reciprocal space.
    pub fn write_kpoints(&self) -> Result<()> {
        let kpoints_config = &self.toml_contents["vasp"]["kpoints"];
        let scheme: KpointsScheme = kpoints_config["scheme"]
            .as_str()
            .unwrap()
            .parse()
            .map_err(|_| eyre::eyre!("Unrecognized Kpoints scheme"))?;

        let kpoints = match kpoints_config.get("mesh") {
            Some(m) => {
                let mesh = [
                    m[0].as_integer().unwrap() as u32,
                    m[1].as_integer().unwrap() as u32,
                    m[2].as_integer().unwrap() as u32,
                ];
                Kpoints::new(scheme, mesh)
            }
            None => {
                let density = kpoints_config["density"]
                    .as_float()
                    .expect("Please specify mesh or k-points density");
                let lattice = Poscar::from_file("POSCAR")?.lattice;
                Kpoints::from_density(scheme, density, lattice)
            }
        };

        let kpoints_str = kpoints.to_string();
        let kpoints_path = format!("{}/KPOINTS", self.job_dir);
        std::fs::write(kpoints_path, kpoints_str)?;
        Ok(())
    }
}
