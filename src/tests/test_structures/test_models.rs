use crate::libs::structures::models::{Galaxy, Output};

#[test]
fn test_galaxy_new() {
    let obj: Galaxy = Galaxy::new();
    assert_eq!(obj.name, "defaultName");
    assert_eq!(obj.distance, 0.0);
    assert_eq!(obj.velocity, 0.0);
}

#[test]
fn test_galaxy_create() {
    let name = String::from("Galaxy 1");
    let distance: f64 = 34.0;
    let velocity: f64 = 14.0;

    let galaxy: Galaxy = Galaxy::create(Some(name.clone()), Some(distance), Some(velocity));
    assert_eq!(galaxy.name, name);
    assert_eq!(galaxy.distance, distance);
    assert_eq!(galaxy.velocity, velocity);
}

#[test]
fn test_galaxy_update() {
    let mut galaxy: Galaxy = Galaxy::new();

    let name: Option<String> = Some(String::from("Galaxy 1"));
    let distance: Option<f64> = Some(14.0 as f64);
    let velocity: Option<f64> = Some(10.0 as f64);
    galaxy.update(name.clone(), distance, velocity);
    assert_eq!(galaxy.name, name.unwrap());
    assert_eq!(galaxy.distance, distance.unwrap());
    assert_eq!(galaxy.velocity, velocity.unwrap());
}

#[test]
fn test_galaxy_h0() {
    let name: String = String::from("Galaxy 1");
    let distance: f64 = 34.0;
    let velocity: f64 = 14.0;

    let galaxy: Galaxy = Galaxy::create(Some(name.clone()), Some(distance), Some(velocity));
    let h0: f64 = galaxy.h0();
    assert_eq!(h0, (velocity / distance));
}

#[test]
fn test_output_new() {
    let obj: Output = Output::new();
    assert_eq!(obj.h0, 0.0);
    assert_eq!(obj.age, 0.0);
}

#[test]
fn test_output_create() {
    let h0: f64 = 14.0;
    let age: f64 = 34.0;

    let obj: Output = Output::create(h0, age);
    assert_eq!(obj.h0, h0);
    assert_eq!(obj.age, age);
}
