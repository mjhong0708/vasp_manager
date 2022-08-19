use super::core::{Kpoints, KpointsScheme};
use crate::poscar::Lattice;

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
