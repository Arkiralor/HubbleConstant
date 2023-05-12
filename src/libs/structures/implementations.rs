//! Module to store extra implementations of the structs defined in `models.rs`.

use std::process::{ExitCode, Termination};

use crate::libs::structures::models::Output;

impl Termination for Output{
    //! Implementation of the Termination trait for the Output struct.
    fn report(self) -> ExitCode{
        //! Report the output of the program.
        return Into::into(0);
    }
}