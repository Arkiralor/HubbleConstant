#![allow(warnings)]

use rocket::{form::validate::contains, serde::json::*, *};

mod libs;

use libs::constants::get_project_root;
use libs::structures::models::{AgeRequest, Galaxy, HubbleRequest, IndexSchema, Output};
use libs::utils::console_handler::{get_args, get_file_path_from_args, get_show};
use libs::utils::files_handler::read_data;
use libs::utils::hubble_handler::{calculate_age, get_h0};
use libs::utils::misc_handler::{format_f64, print_disclaimers};

#[get("/")]
fn index() -> Json<IndexSchema> {
    let resp = format!("Hello, world!");
    Json(IndexSchema::create(Some(resp)))
}

#[post("/hubble-from-data", data = "<req>")]
fn hubble_from_data(req: Json<HubbleRequest>) -> Json<Output> {
    let data: Vec<Galaxy> = match read_data(&req.file_path, req.show) {
        Ok(val) => val,
        Err(_) => panic!("Error while reading data file at {}.", req.file_path),
    };
    let h0: f64 = match get_h0(&data) {
        Ok(val) => val,
        Err(_) => panic!("Error while calculating Hubble Constant."),
    };

    let age: f64 = match calculate_age(h0) {
        Ok(val) => val,
        Err(_) => panic!("Error while calculating age of the universe."),
    };

    Json(Output::create(Some(h0), Some(age)))
}

#[post("/age-from-hubble", data = "<req>")]
fn age_from_hubble(req: Json<AgeRequest>) -> Json<Output> {
    let age: f64 = match calculate_age(req.h0) {
        Ok(val) => val,
        Err(a) => panic!("{}", a),
    };

    Json(Output::create(Some(req.h0), Some(age)))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![hubble_from_data])
        .mount("/", routes![age_from_hubble])
}

#[cfg(test)]
mod tests;
