use serde::Serialize;
use serde_json::{to_string_pretty, Value};
use std::fs::create_dir_all;

use crate::utils::{concat_path, create_file, FileType};

pub fn create_structure(json_node: &Value, curr_path: &str) {
    match json_node {
        Value::Object(obj) => {
            create_gist(curr_path, &Value::Object(obj.clone()));

            for (key, value) in obj {
                let dir = concat_path(&[curr_path, key]);
                create_dir_all(&dir).expect("Could not create directory");

                create_structure(value, &dir)
            }
        }
        Value::Array(arr) => {
            create_gist(curr_path, &Value::Array(arr.clone()));

            for (idx, value) in arr.iter().enumerate() {
                let dir = concat_path(&[curr_path, (idx + 1).to_string().as_str()]);
                create_dir_all(&dir).expect("Could not create directory");

                create_structure(value, &dir)
            }
        }
        Value::String(val) => create_file(curr_path, FileType::String(val), None),
        Value::Number(val) => create_file(curr_path, FileType::Number(val), None),
        Value::Bool(val) => create_file(curr_path, FileType::Bool(val), None),
        Value::Null => create_file(curr_path, FileType::Null(), None),
    }
}

fn create_gist<T: Serialize>(curr_path: &str, content: &T) {
    let json_str = to_string_pretty(content).unwrap();
    create_file(curr_path, FileType::RawStr(".gist.json"), Some(&json_str));
}
