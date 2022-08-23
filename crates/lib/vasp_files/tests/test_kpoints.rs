use vasp_files::kpoints::{Kpoints, KpointsScheme};
use vasp_files::poscar::Poscar;

const POSCAR_TEST_BULK: &'static str = "tests/data/POSCAR";

#[test]
fn test_kpoints() {
    let kpoints = Kpoints::new(KpointsScheme::Gamma, [3, 11, 7]);
    println!("{}", kpoints);
    assert_eq!(kpoints.to_string(), "Autometic mesh\n 0\nGamma\n3  11  7\n0  0  0\n");
}

#[test]
fn test_kpoints_from_density() {
    let poscar = Poscar::from_file(POSCAR_TEST_BULK).unwrap();
    let kpoints = Kpoints::from_density(KpointsScheme::Gamma, 4.0, poscar.lattice);
    println!("{}", kpoints);
}
