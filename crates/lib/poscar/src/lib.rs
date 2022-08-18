pub mod geometry;
pub mod io;

use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lattice {
    pub a: [f64; 3],
    pub b: [f64; 3],
    pub c: [f64; 3],
}

impl Lattice {
    pub fn new(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> Self {
        Lattice { a, b, c }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoordinateSystem {
    Cartesian,
    Direct,
}

impl FromStr for CoordinateSystem {
    type Err = io::PoscarParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.to_ascii_lowercase().starts_with('c') => Ok(CoordinateSystem::Cartesian),
            s if s.to_ascii_lowercase().starts_with('d') => Ok(CoordinateSystem::Direct),
            _ => Err(io::PoscarParseError::UnknownCoordinateSystem(s.to_string())),
        }
    }
}

impl Display for CoordinateSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CoordinateSystem::Cartesian => write!(f, "Cartesian"),
            CoordinateSystem::Direct => write!(f, "Direct"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectiveDynamics(bool);

impl FromStr for SelectiveDynamics {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "T" => Ok(SelectiveDynamics(true)),
            "F" => Ok(SelectiveDynamics(false)),
            _ => Err(format!("Invalid selective dynamics: {}", s)),
        }
    }
}

impl Display for SelectiveDynamics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.0 { "T" } else { "F" })
    }
}

#[derive(Debug)]
pub struct Poscar {
    pub lattice: Lattice,
    pub species: Vec<String>,
    pub num_species: Vec<usize>,
    pub chemical_symbols: Vec<String>,
    pub positions: Vec<[f64; 3]>,
    pub coord_type: CoordinateSystem,
    pub selective_dynamics: Option<Vec<[SelectiveDynamics; 3]>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_no_seldyn() {
        let poscar = Poscar::from_file("test_data/POSCAR").unwrap();
        println!("{}", poscar);
    }

    #[test]
    fn test_read_seldyn() {
        let poscar = Poscar::from_file("test_data/POSCAR_slab").unwrap();
        println!("{}", poscar);
    }
}
