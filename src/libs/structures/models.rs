//! Module to store all the models (structs) used in the project.
use std::fmt::Display;

use serde::Deserialize;
use serde::Serialize;

use crate::libs::constants::{DISTANCE_ERROR_MSG, NULL_VEC_ERROR, VELOCITY_ERROR_MSG};

#[derive(Deserialize, Serialize)]
pub struct IndexSchema {
    msg: String,
}

impl IndexSchema {
    pub fn create(msg: Option<String>) -> Self {
        let data: String = match msg {
            Some(val) => val,
            None => "Hello, World!".to_string(),
        };

        Self { msg: data }
    }
}

#[derive(Deserialize, Serialize)]
pub struct HubbleRequest {
    pub file_path: String,
    pub show: bool,
}

impl HubbleRequest {
    pub fn create(file_path: Option<String>, show: Option<bool>) -> Self {
        let file_path: String = match file_path {
            Some(val) => val,
            None => String::from(""),
        };

        let show: bool = match show {
            Some(val) => val,
            None => false,
        };

        Self { file_path, show }
    }

    pub fn update(&mut self, file_path: Option<String>, show: Option<bool>) {
        self.file_path = match file_path {
            Some(val) => val,
            None => self.file_path.clone(),
        };

        self.show = match show {
            Some(val) => val,
            None => self.show.clone(),
        };
    }
}

#[derive(Deserialize, Serialize)]
pub struct AgeRequest {
    pub h0: f64,
}

impl AgeRequest {
    pub fn create(h0: Option<f64>) -> Self {
        let h0: f64 = match h0 {
            Some(val) => val,
            None => 0.0 as f64,
        };

        Self { h0 }
    }

    pub fn update(&mut self, h0: Option<f64>) {
        self.h0 = match h0 {
            Some(val) => val,
            None => self.h0.clone(),
        };
    }
}

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

    pub fn create_many(
        data: Vec<(Option<String>, Option<f64>, Option<f64>)>,
    ) -> Result<Vec<Galaxy>, String> {
        //! Create multiple `Galaxy` objects via data provided.
        //!
        //! ### Args:
        //!
        //!     [
        //!         1. (name: Option<String>, distance: Option<f64>, velocity: Option<f64>)
        //!         2. (name: Option<String>, distance: Option<f64>, velocity: Option<f64>)
        //!         3. (name: Option<String>, distance: Option<f64>, velocity: Option<f64>)
        //!         .
        //!         .
        //!         n. (name: Option<String>, distance: Option<f64>, velocity: Option<f64>)
        //!     ]
        //!
        //! ### Returns:
        //!     
        //!     Vec<Galaxy>
        if data.is_empty() {
            return Err(NULL_VEC_ERROR.to_owned());
        }

        let mut results: Vec<Galaxy> = Vec::new();

        for item in data.iter() {
            results.push(Galaxy::create(
                item.0.clone(), // This variable needs to be cloned as `String` variables are stored in the Heap and not the stack.
                item.1,
                item.2,
            ))
        }

        Ok(results)
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

    pub fn create(h0: Option<f64>, age: Option<f64>) -> Output {
        //! Create a new output object from the given data.
        let h0: f64 = match h0 {
            Some(val) => val,
            None => 0.0 as f64,
        };

        let age: f64 = match age {
            Some(val) => val,
            None => 0.0 as f64,
        };

        Output { h0, age }
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
