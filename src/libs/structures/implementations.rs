//! Module to store extra implementations of the structs defined in `models.rs`.

use std::process::{ExitCode, Termination};

use crate::libs::structures::models::{Galaxy, Output};


impl Clone for Galaxy {
    //! Implementation of the Clone trait for the Galaxy struct.
    fn clone(&self) -> Self {
        //! Clone the Galaxy struct.
        return Galaxy {
            name: self.name.clone(),
            distance: self.distance.clone(),
            velocity: self.velocity.clone(),
        };
    }
}

impl Termination for Output {
    //! Implementation of the Termination trait for the Output struct.
    fn report(self) -> ExitCode {
        //! Report the output of the program.
        return Into::into(0);
    }
}