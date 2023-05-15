#![allow(warnings)]

mod libs;

use std::error::Error;
use libs::utils::console_handler::{get_args, get_file_path_from_args};
use libs::structures::models::{Galaxy, Output};
use libs::utils::files_handler::read_data;
use libs::utils::hubble_handler::{get_h0, calculate_age};
use libs::utils::misc_handler::format_f64;

fn main() -> Result<Output, Box<dyn Error>>{
    let args: Vec<String> = get_args();
    let file_path: String = get_file_path_from_args(&args);

    let data: Vec<Galaxy> = read_data(&file_path);
    let h0: f64 = get_h0(data);
    let age: f64 = calculate_age(h0);

    println!("DISCLAIMER: The values calculated by this program are based on the data provided by the user.");
    println!("DISCLAIMER: There might be floating point precision errors.");

    println!("Data File Used:\t'{}'", file_path);
    println!("Hubble Constant was calculated to be:\t{} km/s/Mpc", format_f64(h0));
    println!("Age of the Universe:\t{} years", format_f64(age));

    let output: Output = Output::create(h0, age);
    let return_value: Result<Output, Box<dyn Error>> = Ok(output);
    return return_value;
}
