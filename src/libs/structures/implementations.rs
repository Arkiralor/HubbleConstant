//! Module to store extra implementations of the structs defined in `models.rs`.

use std::{
    fmt::{Debug, Display, Formatter, Result},
    process::{ExitCode, Termination},
};

use crate::libs::structures::models::{Galaxy, Output};

impl Clone for Galaxy {
    //! Implementation of the Clone trait for the Galaxy struct.
    fn clone(&self) -> Self {
        //! Clone the Galaxy struct.
        Galaxy {
            name: self.name.clone(),
            distance: self.distance,
            velocity: self.velocity,
        }
    }
}

impl Display for Galaxy {
    //! Implementation of the Display trait for the Galaxy struct.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        //! Format the Galaxy struct for printing.
        write!(f, "{{ \"name\": \"{name}\", \"distance\": {distance}, \"velocity\": {velocity} , \"h0\": {h0} }}" , name=self.name, distance=self.distance, velocity=self.velocity, h0=self.h0())
    }
}

impl Debug for Galaxy {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        //! Format the Galaxy struct for printing.
        write!(f, "{{ \"name\": \"{name}\", \"distance\": {distance}, \"velocity\": {velocity} , \"h0\": {h0} }}" , name=self.name, distance=self.distance, velocity=self.velocity, h0=self.h0())
    }
}

impl Termination for Output {
    //! Implementation of the Termination trait for the Output struct.
    fn report(self) -> ExitCode {
        //! Report the output of the program.
        Into::into(0)
    }
}
