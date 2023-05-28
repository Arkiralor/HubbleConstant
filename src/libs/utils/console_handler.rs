use std::{env, error::Error};

use crate::libs::constants::MAX_ARGS;

pub fn get_args() -> Result<Vec<String>, String> {
    //! Get the command line arguments passed to the program.
    let args: Vec<String> = env::args().collect();
    if !check_args(&args) {
        panic!("Too many/few arguments passed to the program.");
    }
    return Ok(args);
}

fn check_args(args: &Vec<String>) -> bool {
    //! Check if the arguments passed to the program are valid.
    if args.len() > MAX_ARGS || args.len() == 0 {
        return false;
    } else {
        return true;
    }
}

pub fn get_file_path_from_args(args: &Vec<String>) -> Result<String, String> {
    //! Get the file path from the command line arguments.
    let file_path: String = args[1].clone();
    return Ok(file_path);
}

pub fn get_show(args: &Vec<String>) -> Result<bool, String> {
    //! Get the value of the `show` flag from the command line arguments.
    if args.len() == 3 {
        if args[2] == "show" {
            return Ok(true);
        } else {
            return Err("Invalid flag passed.".to_string());
        }
    } else {
        return Ok(false);
    }
}
