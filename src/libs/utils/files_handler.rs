use std::fs::File;
use std::path::Path;

use crate::libs::structures::models::Galaxy;

pub fn read_data(file_path: &str, show:bool) -> Result<Vec<Galaxy>, String> {
    //! Read the json file and parse into Galaxy objects.
    let panic_msg: String = format!("File '{}' not found.", file_path);

    let file_obj: File = File::open(Path::new(file_path)).expect(&panic_msg);
    let data: Vec<Galaxy> = serde_json::from_reader(file_obj).expect("Error while eading file");
    
    if show {
        println!("Galaxies:\t{:?}", data);
    }
    return Ok(data);
}
