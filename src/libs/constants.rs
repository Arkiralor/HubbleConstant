//! Module file to contain all the constants used in the project.

use std::env;
use std::path::PathBuf;

pub const MAX_ARGS: usize = 3; // Maximum number of arguments that can be passed to the program.
pub const MAX_FLOAT_DECIMALS: usize = 12; // Maximum number of decimal places to be displayed for a float value.

pub const KM_IN_MPC: f64 = 3.08E+19; // 1 Megaparsec = 3.08 x 10^19 km
pub const SEC_IN_YEAR: f64 = 3.15576E+7; // 1 Year = 3.15576 x 10^7 seconds.

pub const DISCLAIMER_1: &str =
    "DISCLAIMER #1: The values calculated by this program are based on the data provided by the user.\
    \n\tThe accuracy and precision are dependent upon the data provided in the `.json` file.\
    \n\tFor higher accuracy and precision, please provide more data points in the `.json` file.\n\n";
pub const DISCLAIMER_2: &str = "DISCLAIMER #2: There might be floating point precision errors as the floating point values used are 64-bit in size.";

pub fn get_project_root() -> Result<PathBuf, String> {
    //! Get the root directory of the project.
    //!
    //! _Technically, this is not exactly a constant, but it can be treated as one for the purposes of this codebase._

    let mut path: PathBuf = match env::current_dir() {
        Ok(val) => val,
        Err(s) => return Err(s.to_string()),
    };

    return Ok(path);
}

pub const DISTANCE_ERROR_MSG: &str =
    "The distance of a far-away galaxy from Terra cannot be equal or less than 0 Megaparsecs.";
pub const VELOCITY_ERROR_MSG: &str = "For galaxies where universal expansion is applicable and hence are used to calculate the Hubble Constant, the Helioradial velocity of recession from Terra cannot be equal or less than 0 Kilometres per Second.";
pub const NULL_VEC_ERROR: &str = "The provided vactor of datatype `T` is empty.";
