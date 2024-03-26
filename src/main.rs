mod core;
mod utils;

use core::*;
use utils::*;

use chrono::Local;
use std::fs::create_dir_all;

fn main() {
    let temp_path = get_temp_dir().unwrap();

    let timestamp = Local::now().format("%Y-%m-%d_%H:%M:%S").to_string();
    let path = concat_path(&[&temp_path, &timestamp]);
    create_dir_all(&path).expect("Could not create directory");

    let json = get_json_input();
    create_structure(&json, &path);
}
