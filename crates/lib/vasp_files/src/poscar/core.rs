use super::impl_poscar;

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

#[derive(Debug)]
pub struct LatticeParams {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
}

impl LatticeParams {
    #[rustfmt::skip]
    pub fn new(a: f64, b: f64, c: f64, alpha: f64, beta: f64, gamma: f64) -> Self {
        LatticeParams { a, b, c, alpha, beta, gamma }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoordinateSystem {
    Cartesian,
    Direct,
}

impl FromStr for CoordinateSystem {
    type Err = impl_poscar::PoscarParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.to_ascii_lowercase().starts_with('c') => Ok(CoordinateSystem::Cartesian),
            s if s.to_ascii_lowercase().starts_with('d') => Ok(CoordinateSystem::Direct),
            _ => Err(impl_poscar::PoscarParseError::UnknownCoordinateSystem(s.to_string())),
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
