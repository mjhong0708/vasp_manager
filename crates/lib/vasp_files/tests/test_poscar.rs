use vasp_files::poscar::{Lattice, LatticeParams, Poscar};

#[test]
fn test_read_no_seldyn() {
    let poscar = Poscar::from_file("tests/data/POSCAR").unwrap();
    println!("{}", poscar);
}

#[test]
fn test_read_seldyn() {
    let poscar = Poscar::from_file("tests/data/POSCAR_slab").unwrap();
    println!("{}", poscar);
}

#[test]
fn test_lattice_params() {
    let lattice = Lattice {
        a: [1.0, 0.0, 0.0],
        b: [0.0, 1.0, 0.0],
        c: [0.0, 0.0, 1.0],
    };
    let lattice_params = LatticeParams::from(lattice);
    assert_eq!(lattice_params.a, 1.0);
    assert_eq!(lattice_params.b, 1.0);
    assert_eq!(lattice_params.c, 1.0);
    assert_eq!(lattice_params.alpha, 90.0);
    assert_eq!(lattice_params.beta, 90.0);
    assert_eq!(lattice_params.gamma, 90.0);
}

#[test]
fn test_reciprocal_lattice() {
    let pi = std::f64::consts::PI;
    let poscar = Poscar::from_file("tests/data/POSCAR_slab").unwrap();
    let reciprocal_lattice = poscar.get_reciprocal_lattice();

    assert!((reciprocal_lattice.a[0] / (2.0 * pi) - 1.80384383e-01) < 1e-6);
    assert!((reciprocal_lattice.a[1] / (2.0 * pi) - -1.04144972e-01) < 1e-6);
    assert!((reciprocal_lattice.a[2] / (2.0 * pi) - 0.0) < 1e-6);

    assert!((reciprocal_lattice.b[0] / (2.0 * pi) - 0.0) < 1e-6);
    assert!((reciprocal_lattice.b[1] / (2.0 * pi) - 2.08289944e-01) < 1e-6);
    assert!((reciprocal_lattice.b[2] / (2.0 * pi) - 0.0) < 1e-6);

    assert!((reciprocal_lattice.c[0] / (2.0 * pi) - 0.0) < 1e-6);
    assert!((reciprocal_lattice.c[1] / (2.0 * pi) - 0.0) < 1e-6);
    assert!((reciprocal_lattice.c[2] / (2.0 * pi) - 4.15751110e-02) < 1e-6);

    println!("{:?}", reciprocal_lattice.lattice_params());
}
