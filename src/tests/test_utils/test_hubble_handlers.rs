use crate::libs::constants::{KM_IN_MPC, SEC_IN_YEAR};
use crate::libs::structures::models::Galaxy;
use crate::libs::utils::hubble_handler::{calculate_age, get_h0};

#[test]
fn test_get_h0() {
    let data: Vec<Galaxy> = vec![
        Galaxy::create(Some(String::from("Galaxy 1")), Some(34.0), Some(14.0)),
        Galaxy::create(Some(String::from("Galaxy 2")), Some(14.0), Some(10.0)),
        Galaxy::create(Some(String::from("Galaxy 3")), Some(10.0), Some(4.0)),
    ];
    let h0: f64 = get_h0(&data).unwrap();
    let mut raw_ho_list: Vec<f64> = Vec::new();
    for item in data.iter() {
        raw_ho_list.push(item.velocity / item.distance);
    }
    assert_eq!(
        h0,
        (raw_ho_list.iter().sum::<f64>() / raw_ho_list.len() as f64)
    );
}

#[test]
fn test_calculate_age() {
    let h0: f64 = 0.5;
    let age: f64 = calculate_age(h0).unwrap();
    let raw_age = (1.0 / (h0 / KM_IN_MPC)) / SEC_IN_YEAR;
    assert_eq!(age, raw_age);
}
