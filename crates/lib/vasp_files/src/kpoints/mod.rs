mod core;
mod impl_kpoints;
pub use self::core::{Kpoints, KpointsError, KpointsScheme};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poscar::Poscar;

    #[test]
    fn test_kpoints() {
        let kpoints = Kpoints::new(KpointsScheme::Gamma, [3, 11, 7]);
        println!("{}", kpoints);
        assert_eq!(kpoints.to_string(), "Autometic mesh\n 0\nGamma\n3  11  7\n0  0  0\n");
    }

    #[test]
    fn test_kpoints_from_density() {
        let poscar = Poscar::from_file("../poscar/test_data/POSCAR_slab").unwrap();
        let kpoints = Kpoints::from_density(KpointsScheme::Gamma, 4.0, poscar.lattice);
        println!("{}", kpoints);
    }
}
