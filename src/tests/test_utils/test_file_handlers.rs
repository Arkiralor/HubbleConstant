use crate::libs::constants::get_project_root;

use std::fs;
use std::path::Path;

const DATA_DIR: &str = "data";
const DATA_FILE: &str = "galaxies.json";

#[test]
pub fn test_example_file_exists() {
    let mut file_path = match get_project_root() {
        Ok(val) => val.join(DATA_DIR).join(DATA_FILE),
        Err(s) => panic!("{}", s),
    };

    let path = match Path::new(&file_path).canonicalize() {
        Ok(val) => val,
        Err(s) => panic!("{}", s),
    };
    assert!(
        path.exists(),
        "File path '{:?}' does not exist",
        &path.display()
    );
}
