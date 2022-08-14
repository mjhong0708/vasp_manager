use crate::{Lattice, Poscar, SelectiveDynamics};
use std::fmt::Display;
use std::fs::read_to_string;
use std::str::FromStr;

impl FromStr for Poscar {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next().unwrap();

        let scaling_factor = lines.next().unwrap().trim().parse::<f64>().unwrap();

        let lattice = {
            let mut lattice = [[0.0; 3]; 3];
            for i in 0..3 {
                let line = lines.next().unwrap();
                let mut tokens = line.trim().split_whitespace();
                for j in 0..3 {
                    lattice[i][j] = tokens.next().unwrap().parse::<f64>().unwrap() * scaling_factor;
                }
            }
            Lattice::new(lattice[0], lattice[1], lattice[2])
        };

        let species = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let num_species = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let chemical_symbols = species
            .iter()
            .zip(&num_species)
            .flat_map(|(s, n)| std::iter::repeat(s.clone()).take(*n))
            .collect::<Vec<_>>();

        let coord_or_selective_dynamics = lines.next().unwrap();
        let mut coord_type = coord_or_selective_dynamics.to_string();
        let is_selective_dynamics = coord_or_selective_dynamics.starts_with("S");
        if is_selective_dynamics {
            coord_type = lines.next().unwrap().to_string();
        }

        let mut positions = Vec::new();
        let mut selective_dynamics = Vec::new();
        for _ in 0..num_species.iter().sum::<usize>() {
            let line = lines.next().unwrap();
            let mut tokens = line.split_whitespace();
            let x = tokens.next().unwrap().parse::<f64>().unwrap();
            let y = tokens.next().unwrap().parse::<f64>().unwrap();
            let z = tokens.next().unwrap().parse::<f64>().unwrap();
            positions.push([x, y, z]);
            if is_selective_dynamics {
                let fix_x: SelectiveDynamics = tokens.next().unwrap().parse().unwrap();
                let fix_y: SelectiveDynamics = tokens.next().unwrap().parse().unwrap();
                let fix_z: SelectiveDynamics = tokens.next().unwrap().parse().unwrap();
                selective_dynamics.push([fix_x, fix_y, fix_z]);
            }
        }

        Ok(Poscar {
            lattice,
            species,
            num_species,
            chemical_symbols,
            positions,
            coord_type: coord_type.parse().unwrap(),
            selective_dynamics: if !selective_dynamics.is_empty() {
                Some(selective_dynamics)
            } else {
                None
            },
        })
    }
}

impl Poscar {
    pub fn from_file(path: &str) -> Self {
        let contents = read_to_string(path).unwrap();
        Poscar::from_str(&contents).unwrap()
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
