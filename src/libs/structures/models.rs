//! Module to store all the models (structs) used in the project.
use std::fmt::Display;


use serde::Deserialize;
use serde::Serialize;

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
        let name: String = String::from("");
        let obj: Galaxy = Galaxy {
            name: name,
            distance: 0.0 as f64,
            velocity: 0.0 as f64,
        };
        return obj;
    }

    pub fn create(name: Option<String>, distance: Option<f64>, velocity: Option<f64>) -> Galaxy {
        //! Create a new galaxy object from the given data.      
        
        let obj: Galaxy = Galaxy {
            name: name.unwrap_or(String::from("defaultName")),
            distance: distance.unwrap_or(0.0 as f64),
            velocity: velocity.unwrap_or(0.0 as f64),
        };
        return obj;
    }

    pub fn update(&mut self, name: Option<String>, distance: Option<f64>, velocity: Option<f64>) {
        //! Update the values for a galaxy object.
        self.name = name.unwrap_or(String::from(""));
        self.distance = distance.unwrap_or(0.0 as f64);
        self.velocity = velocity.unwrap_or(0.0 as f64);
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
        self.h0 = h0.unwrap_or(0.0 as f64);
        self.age = age.unwrap_or(0.0 as f64);
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
