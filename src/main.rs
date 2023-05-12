#![allow(warnings)]

mod libs;

fn main() -> libs::structures::models::Output{
    let args: Vec<String> = libs::utils::console_handler::get_args();
    let file_path: String = libs::utils::console_handler::get_file_path_from_args(&args);

    let data: Vec<libs::structures::models::Galaxy> = libs::utils::files_handler::read_data(&file_path);
    let h_0: f64 = libs::utils::hubble_handler::get_h0(data);
    let age: f64 = libs::utils::hubble_handler::calculate_age(h_0);

    println!("DISCLAIMER: The values calculated by this program are based on the data provided by the user.");
    println!("DISCLAIMER: There might be floating point precision errors.");
    println!("\n\n\n");

    println!("Data file: {}", file_path);
    println!("Hubble Constant was calculated to be: {} km/s/Mpc", libs::utils::misc_handler::format_f64(h_0));
    println!("Age of the Universe: {} years", libs::utils::misc_handler::format_f64(age));

    let output: libs::structures::models::Output = libs::structures::models::Output::create(h_0, age);
    return output;
}
