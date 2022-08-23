use std::str::FromStr;

use vasp_files::incar::{Incar, IncarTag};

#[test]
fn test_incar_tag() {
    let tag = IncarTag::from_str("ISPIN = 1").unwrap();
    assert_eq!(tag.name(), "ISPIN");
    if let IncarTag::Integer(n, v) = tag {
        assert_eq!(n, "ISPIN");
        assert_eq!(v, 1);
    } else {
        panic!("ISPIN is not a boolean");
    }
}

#[test]
fn test_read_incar() {
    let incar = Incar::from_file("tests/data/INCAR").unwrap();
    println!("{}", incar.to_string());
}
