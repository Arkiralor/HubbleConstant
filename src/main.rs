#![allow(warnings)]

use std::path::Path;

mod libs;

use libs::constants::get_project_root;
use libs::structures::models::{Galaxy, Output};
use libs::utils::console_handler::{get_args, get_file_path_from_args};
use libs::utils::files_handler::read_data;
use libs::utils::hubble_handler::{calculate_age, get_h0};
use libs::utils::misc_handler::{format_f64, print_disclaimers};
use std::error::Error;

fn main() -> Result<Output, Box<dyn Error>> {
    let args: Vec<String> = match get_args() {
        Ok(val) => val,
        Err(_) => panic!("Error while getting command line arguments."),
    };

    let file_path: String = match get_file_path_from_args(&args) {
        Ok(val) => val,
        Err(_) => get_project_root()
            .unwrap()
            .join("data")
            .join("galaxies.csv")
            .to_str()
            .unwrap()
            .to_string(),
    };

    let data: Vec<Galaxy> = match read_data(&file_path) {
        Ok(val) => val,
        Err(_) => panic!("Error while reading data file."),
    };
    let h0: f64 = match get_h0(data) {
        Ok(val) => val,
        Err(_) => panic!("Error while calculating Hubble Constant."),
    };
    let age: f64 = match calculate_age(h0) {
        Ok(val) => val,
        Err(_) => panic!("Error while calculating age of the universe."),
    };

    print_disclaimers();

    println!("Data File Used:\t'{}'", file_path);
    println!(
        "Hubble Constant was calculated to be:\t{} km/s/Mpc",
        format_f64(h0)
    );
    println!("Age of the Universe:\t{} years", format_f64(age));

    let output: Output = Output::create(h0, age);
    let return_value: Result<Output, Box<dyn Error>> = Ok(output);
    return return_value;
}
