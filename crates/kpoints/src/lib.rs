use poscar::Lattice;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KpointsError {
    pub message: String,
}

impl Display for KpointsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub enum KpointsScheme {
    Gamma,
    MonkhorstPack,
}

impl Display for KpointsScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KpointsScheme::Gamma => write!(f, "Gamma"),
            KpointsScheme::MonkhorstPack => write!(f, "Monkhorst-Pack"),
        }
    }
}

impl FromStr for KpointsScheme {
    type Err = KpointsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.to_ascii_lowercase().starts_with('g') => Ok(KpointsScheme::Gamma),
            s if s.to_ascii_lowercase().starts_with('m') => Ok(KpointsScheme::MonkhorstPack),
            _ => Err(KpointsError {
                message: format!("Unrecognized Scheme: {}", s),
            }),
        }
    }
}

pub struct Kpoints {
    pub scheme: KpointsScheme,
    pub mesh: [u32; 3],
}

impl Kpoints {
    pub fn default() -> Kpoints {
        Kpoints {
            scheme: KpointsScheme::Gamma,
            mesh: [1, 1, 1],
        }
    }

    pub fn new(scheme: KpointsScheme, mesh: [u32; 3]) -> Kpoints {
        Kpoints { scheme, mesh }
    }

    pub fn from_density(scheme: KpointsScheme, density: f64, lattice: Lattice) -> Kpoints {
        let reciprocal_lattice_params = lattice.reciprocal().lattice_params();
        let a = reciprocal_lattice_params.a;
        let b = reciprocal_lattice_params.b;
        let c = reciprocal_lattice_params.c;

        let num_ka = (a * density).round() as u32;
        let num_kb = (b * density).round() as u32;
        let num_kc = (c * density).round() as u32;
        let mesh = [num_ka, num_kb, num_kc];
        Kpoints { scheme, mesh }
    }
}

impl Display for Kpoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Autometic mesh\n 0\n{}\n{}  {}  {}\n0  0  0\n",
            self.scheme, self.mesh[0], self.mesh[1], self.mesh[2]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poscar::Poscar;

    #[test]
    fn test_kpoints() {
        let kpoints = Kpoints::new(KpointsScheme::Gamma, [3, 11, 7]);
        println!("{}", kpoints);
        assert_eq!(kpoints.to_string(), "Autometic mesh\n 0\nGamma\n3  11  7\n0  0  0\n");
    }

    #[test]
    fn test_kpoints_from_density() {
        let poscar = Poscar::from_file("../poscar/test_data/POSCAR_slab");
        let kpoints = Kpoints::from_density(KpointsScheme::Gamma, 4.0, poscar.lattice);
        println!("{}", kpoints);
    }
}
