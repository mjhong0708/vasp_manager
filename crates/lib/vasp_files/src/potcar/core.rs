use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;

const POTCAR_JSON: &str = include_str!("../data/potcar.json");

#[derive(Deserialize)]
pub struct PotcarData {
    pub element: String,
    pub potcar_name: String,
    pub enmax: i32,
    pub recommended: bool,
}

pub enum PotcarMode {
    Recommended,
    Custom(HashMap<String, String>),
}

lazy_static! {
    pub static ref POTCAR_DATA: Vec<PotcarData> =
        serde_json::from_str(POTCAR_JSON).expect("Error in reading potcar.json");
}
