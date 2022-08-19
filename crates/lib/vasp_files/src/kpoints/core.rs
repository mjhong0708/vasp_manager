use std::{fmt::Display, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum KpointsError {
    #[error("Unknown scheme {0}.")]
    UnknownScheme(String),
}

pub struct Kpoints {
    pub scheme: KpointsScheme,
    pub mesh: [u32; 3],
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
            _ => Err(KpointsError::UnknownScheme(s.to_string())),
        }
    }
}
