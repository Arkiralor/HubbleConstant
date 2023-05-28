//! Module file to contain all the constants used in the project.

use std::env;
use std::ops::DerefMut;
use std::path::PathBuf;

pub const MAX_ARGS: usize = 2; // Maximum number of arguments that can be passed to the program.
pub const MAX_FLOAT_DECIMALS: usize = 4; // Maximum number of decimal places to be displayed for a float value.

pub const KM_IN_MPC: f64 = 3.08E+19; // 1 Megaparsec = 3.08 x 10^19 km
pub const SEC_IN_YEAR: f64 = 3.15576E+7; // 1 Year = 3.15576 x 10^7 seconds.

pub const NOTICE: &str = "NOTICE: The program will use the default data file if no file path is provided as an argument.";
pub const DISCLAIMER_1: &str =
    "DISCLAIMER: The values calculated by this program are based on the data provided by the user.";
pub const DISCLAIMER_2: &str = "DISCLAIMER: There might be floating point precision errors.";

pub fn get_project_root() -> Result<PathBuf, String> {
    //! Get the root directory of the project.
    //!
    //! _Technically this is not exactly a constant, but it can be treated as one for the purposes of this codebase._
    let mut path: PathBuf = env::current_dir().unwrap();

    println!("Current Directory: {}", path.display());
    return Ok(path);
}
