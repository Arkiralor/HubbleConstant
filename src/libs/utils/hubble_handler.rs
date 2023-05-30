use std::error::Error;

use crate::libs::constants::{KM_IN_MPC, SEC_IN_YEAR};
use crate::libs::structures::models::Galaxy;

pub fn get_h0(data: &Vec<Galaxy>) -> Result<f64, String> {
    //! Calculate the mean value of the Hubble Constant from
    //! a list of H0s of observed galaxies.
    //!
    //! The Hubble constant can be defined as
    //!
    //! _if a far away object `b` is `x` units away from object `a` (observer), the hubble constant is the relation
    //! between how large the distance `a->b` is and how high the velocity is at which `b` is travelling away from `a`_.
    if data.is_empty() {
        return Err("Empty data list".to_string());
    }

    let _sum: f64 = data.iter().map(|item: &Galaxy| item.h0()).sum();

    if _sum.is_nan() {
        return Err("Invalid Hubble Constant values".to_string());
    }

    Ok(_sum / data.len() as f64)
}

pub fn calculate_age(h0: f64) -> Result<f64, String> {
    //! Calculate the age of the universe given a value for the Hubble Constant.
    //!
    //! Derivation Link: [www.e-education.psu.edu](https://www.e-education.psu.edu/astro801/content/l10_p5.html)
    if h0 <= 0.0 {
        return Err("Hubble Constant must be positive.".to_string());
    }
    let resolved: f64 = h0 / KM_IN_MPC; // This is the Hubble Constant expressed in Km/s/Km i.e, Kilometres per second per Kilometre.
    if resolved <= 0.0 {
        return Err("The Hubble Constant, expressed in KM/s/KM must be positive.".to_string());
    }
    return Ok((1.0 / resolved) / SEC_IN_YEAR);
}
