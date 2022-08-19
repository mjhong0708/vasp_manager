use super::core::{Lattice, LatticeParams};
use nalgebra::Vector3;
use nalgebra::{Matrix3, RowVector3};
use std::fmt::Display;

impl Lattice {
    pub fn lattice_params(self) -> LatticeParams {
        self.into()
    }

    pub fn to_matrix(self) -> Matrix3<f64> {
        let a_vec = RowVector3::new(self.a[0], self.a[1], self.a[2]);
        let b_vec = RowVector3::new(self.b[0], self.b[1], self.b[2]);
        let c_vec = RowVector3::new(self.c[0], self.c[1], self.c[2]);
        Matrix3::from_rows(&[a_vec, b_vec, c_vec])
    }

    pub fn volume(self) -> f64 {
        self.to_matrix().determinant().abs()
    }

    pub fn reciprocal(self) -> Lattice {
        let pi = std::f64::consts::PI;
        let volume = self.volume();

        let mat = self.to_matrix();
        let a_vec = &mat.row(0);
        let b_vec = &mat.row(1);
        let c_vec = &mat.row(2);

        let a_reciprocal = 2.0 * pi / volume * b_vec.cross(&c_vec);
        let b_reciprocal = 2.0 * pi / volume * c_vec.cross(&a_vec);
        let c_reciprocal = 2.0 * pi / volume * a_vec.cross(&b_vec);
        Lattice {
            a: [a_reciprocal[0], a_reciprocal[1], a_reciprocal[2]],
            b: [b_reciprocal[0], b_reciprocal[1], b_reciprocal[2]],
            c: [c_reciprocal[0], c_reciprocal[1], c_reciprocal[2]],
        }
    }
}

impl Display for Lattice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:.9}  {:.9}  {:.9}\n{:.9}  {:.9}  {:.9}\n{:.9}  {:.9}  {:.9}",
            self.a[0], self.a[1], self.a[2], self.b[0], self.b[1], self.b[2], self.c[0], self.c[1], self.c[2]
        )
    }
}

impl From<Lattice> for LatticeParams {
    fn from(lattice: Lattice) -> Self {
        let a_vec = Vector3::new(lattice.a[0], lattice.a[1], lattice.a[2]);
        let b_vec = Vector3::new(lattice.b[0], lattice.b[1], lattice.b[2]);
        let c_vec = Vector3::new(lattice.c[0], lattice.c[1], lattice.c[2]);
        let a = a_vec.norm();
        let b = b_vec.norm();
        let c = c_vec.norm();

        let alpha = b_vec.angle(&c_vec).to_degrees();
        let beta = a_vec.angle(&c_vec).to_degrees();
        let gamma = a_vec.angle(&b_vec).to_degrees();
        LatticeParams::new(a, b, c, alpha, beta, gamma)
    }
}

#[cfg(test)]
mod tests {
    use super::super::core::Poscar;
    use super::{Lattice, LatticeParams};
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
        let poscar = Poscar::from_file("test_data/POSCAR_slab").unwrap();
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
}
