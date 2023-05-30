//! Module to store all the models (structs) used in the project.
use std::fmt::Display;

use serde::Deserialize;
use serde::Serialize;

use crate::libs::constants::{DISTANCE_ERROR_MSG, VELOCITY_ERROR_MSG};

#[derive(Deserialize, Serialize)]
/// The struct will hold the information about a single galaxy as defined in `galaxies.json`
///
/// Members:
///
/// * `name`: String - _Name of the galaxy_.
/// * `distance`: f64 - _Distance of the galaxy from Earth in Megaparsec(s)_.
/// * `velocity`: f64 - _Velocity of the galaxy in Kilometre(s) per Second_.
pub struct Galaxy {
    pub name: String,
    pub distance: f64,
    pub velocity: f64,
}

/// Implementation of the Galaxy struct.
impl Galaxy {
    pub fn new() -> Galaxy {
        //! Create a new blank galaxy object.
        let name: String = String::from("defaultName");
        let distance: f64 = 0.0 as f64;
        let velocity: f64 = 0.0 as f64;

        Galaxy {
            name,
            distance,
            velocity,
        }
    }

    pub fn create(n: Option<String>, d: Option<f64>, v: Option<f64>) -> Galaxy {
        //! Create a new galaxy object from the given data.      

        let name: String = match n {
            Some(val) => val,
            None => String::from("defaultName"),
        };

        let distance: f64 = match d {
            Some(val) => {
                if val <= 0.0 as f64 {
                    panic!("{}", DISTANCE_ERROR_MSG.to_string());
                }
                val
            }
            None => 0.0 as f64,
        };

        let velocity: f64 = match v {
            Some(val) => {
                if val <= 0.0 as f64 {
                    panic!("{}", VELOCITY_ERROR_MSG.to_string());
                }
                val
            }
            None => 0.0 as f64,
        };

        Galaxy {
            name,
            distance,
            velocity,
        }
    }

    pub fn update(&mut self, name: Option<String>, distance: Option<f64>, velocity: Option<f64>) {
        //! Update the values for a galaxy object.

        match name {
            Some(val) => self.name = val,
            _ => {}
        };

        match distance {
            Some(val) => {
                if val <= 0.0 as f64 {
                    panic!("{}", DISTANCE_ERROR_MSG.to_string());
                } else {
                    self.distance = val;
                }
            }
            _ => {}
        };

        match velocity {
            Some(val) => {
                if val <= 0.0 as f64 {
                    panic!("{}", VELOCITY_ERROR_MSG.to_string());
                } else {
                    self.velocity = val;
                }
            }
            _ => {}
        };
    }

    pub fn h0(&self) -> f64 {
        //! Calculate the Hubble Constant from the data of a single galaxy object.
        //!
        //! Derivation Link: [www.e-education.psu.edu](https://www.e-education.psu.edu/astro801/content/l10_p5.html)
        let h0: f64 = &self.velocity / &self.distance;
        return h0;
    }
}

#[derive(Debug, Serialize)]
pub struct Output {
    pub h0: f64,
    pub age: f64,
}

impl Output {
    pub fn new() -> Output {
        //! Create a new blank output object.
        let obj: Output = Output {
            h0: 0.0 as f64,
            age: 0.0 as f64,
        };
        return obj;
    }

    pub fn create(h0: f64, age: f64) -> Output {
        //! Create a new output object from the given data.
        let obj: Output = Output { h0: h0, age: age };
        return obj;
    }

    pub fn update(&mut self, h0: Option<f64>, age: Option<f64>) {
        //! Update the values for an output object.
        self.h0 = match h0 {
            Some(val) => val,
            None => self.h0.clone(),
        };
        self.age = match age {
            Some(val) => val,
            None => self.age.clone(),
        };
    }
}

#[derive(Debug)]
pub struct InvalidInputError(&'static str);
impl std::error::Error for InvalidInputError {}
impl std::fmt::Display for InvalidInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
