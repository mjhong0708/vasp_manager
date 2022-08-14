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

lazy_static! {
    static ref POTCAR_DATA: Vec<PotcarData> = serde_json::from_str(POTCAR_JSON).expect("Error in reading potcar.json");
}

pub fn get_recommended_potcars(elems: &[String]) -> Vec<String> {
    elems
        .iter()
        .map(|e| POTCAR_DATA.iter().find(|p| &p.element == e && p.recommended).unwrap())
        .map(|p| p.potcar_name.to_string())
        .collect()
}

pub fn get_potcars_from_map(elems: &[String], potcar_map: &HashMap<String, String>) -> Vec<String> {
    elems.iter().map(|e| potcar_map.get(e).unwrap().to_string()).collect()
}
