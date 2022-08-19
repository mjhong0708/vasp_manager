use super::{Lattice, Poscar, SelectiveDynamics};
use std::fmt::Display;
use std::fs::read_to_string;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum PoscarParseError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Unknown coordinate system {0}.")]
    UnknownCoordinateSystem(String),
    #[error("Unknown selective dynamics {0}.")]
    BadSelectiveDynamics(String),
    #[error("Malformed POSCAR file.")]
    BadPOSCAR,
}

use PoscarParseError::*;
impl FromStr for Poscar {
    type Err = PoscarParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next().ok_or(BadPOSCAR)?;

        let scaling_factor = lines
            .next()
            .ok_or(BadPOSCAR)?
            .trim()
            .parse::<f64>()
            .map_err(|_| BadPOSCAR)?;

        let lattice = {
            let mut lattice = [[0.0; 3]; 3];
            for i in 0..3 {
                let line = lines.next().ok_or(BadPOSCAR)?;
                let mut tokens = line.trim().split_whitespace();
                for j in 0..3 {
                    lattice[i][j] =
                        tokens.next().ok_or(BadPOSCAR)?.parse::<f64>().map_err(|_| BadPOSCAR)? * scaling_factor;
                }
            }
            Lattice::new(lattice[0], lattice[1], lattice[2])
        };

        let species = lines
            .next()
            .ok_or(BadPOSCAR)?
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let num_species = lines
            .next()
            .ok_or(BadPOSCAR)?
            .split_whitespace()
            .map(|s| s.parse::<usize>().map_err(|_| BadPOSCAR))
            .collect::<Result<Vec<usize>, PoscarParseError>>()?;
        let chemical_symbols = species
            .iter()
            .zip(&num_species)
            .flat_map(|(s, n)| std::iter::repeat(s.clone()).take(*n))
            .collect::<Vec<_>>();

        let coord_or_selective_dynamics = lines.next().ok_or(BadPOSCAR)?;
        let mut coord_type = coord_or_selective_dynamics.to_string();
        let is_selective_dynamics = coord_or_selective_dynamics.starts_with("S");
        if is_selective_dynamics {
            coord_type = lines.next().ok_or(BadPOSCAR)?.to_string();
        }

        let mut positions = Vec::new();
        let mut selective_dynamics = Vec::new();
        for _ in 0..num_species.iter().sum::<usize>() {
            let line = lines.next().ok_or(BadPOSCAR)?;
            let mut tokens = line.split_whitespace();
            let x = tokens.next().ok_or(BadPOSCAR)?.parse::<f64>().map_err(|_| BadPOSCAR)?;
            let y = tokens.next().ok_or(BadPOSCAR)?.parse::<f64>().map_err(|_| BadPOSCAR)?;
            let z = tokens.next().ok_or(BadPOSCAR)?.parse::<f64>().map_err(|_| BadPOSCAR)?;
            positions.push([x, y, z]);
            if is_selective_dynamics {
                let fix_x: SelectiveDynamics = tokens
                    .next()
                    .ok_or(BadPOSCAR)?
                    .parse()
                    .map_err(|s| BadSelectiveDynamics(s))?;
                let fix_y: SelectiveDynamics = tokens
                    .next()
                    .ok_or(BadPOSCAR)?
                    .parse()
                    .map_err(|s| BadSelectiveDynamics(s))?;
                let fix_z: SelectiveDynamics = tokens
                    .next()
                    .ok_or(BadPOSCAR)?
                    .parse()
                    .map_err(|s| BadSelectiveDynamics(s))?;
                selective_dynamics.push([fix_x, fix_y, fix_z]);
            }
        }

        Ok(Poscar {
            lattice,
            species,
            num_species,
            chemical_symbols,
            positions,
            coord_type: coord_type.parse()?,
            selective_dynamics: if !selective_dynamics.is_empty() {
                Some(selective_dynamics)
            } else {
                None
            },
        })
    }
}

impl Poscar {
    pub fn from_file(path: &str) -> Result<Self, PoscarParseError> {
        let contents = read_to_string(path).map_err(|_| FileNotFound(path.to_string()))?;
        Poscar::from_str(&contents)
    }
}

impl Display for Poscar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Poscar")?;
        writeln!(f, "1.0")?;
        writeln!(f, "{}", self.lattice)?;
        writeln!(f, "{}", self.species.join(" "))?;
        writeln!(
            f,
            "{}",
            self.num_species
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )?;
        if self.selective_dynamics.is_some() {
            writeln!(f, "Selective dynamics")?;
        }
        writeln!(f, "{}", self.coord_type)?;
        for i in 0..self.positions.len() {
            write!(
                f,
                "{:.9}  {:.9}  {:.9}  ",
                self.positions[i][0], self.positions[i][1], self.positions[i][2]
            )?;
            if let Some(sd) = &self.selective_dynamics {
                write!(f, "{}  {}  {}  ", sd[i][0], sd[i][1], sd[i][2])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
