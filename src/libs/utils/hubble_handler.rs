use crate::libs::constants::{KM_IN_MPC, SEC_IN_YEAR};
use crate::libs::structures::models::Galaxy;

pub fn get_h0(data: Vec<Galaxy>)->f64{
    //! Calculate the mean value of the Hubble Constant from
    //! a list of H0s of observed galaxies.
    //! 
    //! The Hubble constant can be defined as 
    //! 
    //! _if a far away object `b` is `x` units away from object `a` (observer), the hubble constant is the relation
    //! between how large the distance `a->b` is and how high the velocity is at which `b` is travelling away from `a`_.
    let mut _sum: f64 = 0.0;

    for item in data.iter(){
        _sum = _sum + item.h0();
    }

    return _sum / data.len() as f64;
}

pub fn calculate_age(h0: f64) -> f64{
    //! Calculate the age of the universe given a value for the Hubble Constant.
    //! 
    //! Derivation Link: [www.e-education.psu.edu](https://www.e-education.psu.edu/astro801/content/l10_p5.html)
    let resolved = h0 / KM_IN_MPC; // This is the Hubble Constant expressed in Km/s/Km i.e, Kilometres per second per Kilometre.
    return (1.0 / resolved) / SEC_IN_YEAR;
}