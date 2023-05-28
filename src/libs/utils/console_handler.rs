use std::{env, error::Error};

use crate::libs::constants::MAX_ARGS;

pub fn get_args() -> Vec<String> {
    //! Get the command line arguments passed to the program.
    let args: Vec<String> = env::args().collect();

    if !check_args(&args) {
        panic!("Too many/few arguments passed to the program.");
    }
    return args;
}

fn check_args(args: &Vec<String>) -> bool {
    //! Check if the arguments passed to the program are valid.
    if args.len() > MAX_ARGS || args.len() <= 0 {
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
