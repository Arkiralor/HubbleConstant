use crate::libs::constants::get_project_root;

use std::fs;
use std::path::Path;

#[test]
pub fn test_example_file_exists() {
    let mut file_path = match get_project_root() {
        Ok(val) => val.join("data").join("galaxies.json"),
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
