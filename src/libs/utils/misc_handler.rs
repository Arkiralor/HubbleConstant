//! Handler functions for miscellaneous tasks.
use crate::libs::constants::{DISCLAIMER_1, DISCLAIMER_2, MAX_FLOAT_DECIMALS};

pub fn format_f64(f: f64) -> Result<String, String> {
    //! Format a float value to a string with a maximum of `MAX_FLOAT_DECIMALS` decimal places.

    let mut format_string: String = format!("{:.*}", MAX_FLOAT_DECIMALS, f);
    Ok(format_string)
}

pub fn unique_elements_vector<T: std::fmt::Debug + std::cmp::PartialEq>(
    _list: Vec<T>,
) -> Result<Vec<T>, String> {
    //! Find all the UNIQUE elements in a given vector of Datatype T;
    //!
    //! _where T has the following attributes:_
    //!
    //! 1. Debug()
    //! 2. PartialEq()
    //!
    //!  __Arguments:__
    //!
    //!  1. _list: Vec<T>
    //!
    //!  __Returns:__
    //!      Vec<T>
    let mut unique_list: Vec<T> = Vec::new();
    for item in _list {
        if !unique_list.contains(&item) {
            unique_list.push(item);
        }
    }
    Ok(unique_list)
}

pub fn print_disclaimers() {
    //! Print the disclaimers.
    println!("\n{}", DISCLAIMER_1);
    println!("{}\n", DISCLAIMER_2);
}
