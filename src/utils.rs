use home::home_dir;
use serde_json::{from_str, Number, Value};
use std::env::args;
use std::env::var as get_env;
use std::fs::{create_dir_all, File};
use std::io::Result;
use std::io::Write;
use std::path::PathBuf;

/**
 Checks if a `/temp` directory does not exist then creates it

 returns the path to the `/temp` directory
*/
pub fn get_temp_dir() -> Result<String> {
    let path: PathBuf = get_env("LSON_PATH")
        .map(|p| p.into())
        .unwrap_or_else(|_| {
            home_dir().expect("Please export LSON_PATH from .bashrc / .zshrc / config.fish")
        })
        .join("lson/temp");

    create_dir_all(&path)?;

    Ok(path.to_str().unwrap().to_owned())
}

/// Parses the first argument as a JSON string
pub fn get_json_input() -> Value {
    let input: String = args().nth(1).expect("Please provide a JSON string");
    return from_str(&input).expect("Provided string is not a valid JSON string");
}

/** Concatenates a list of directory names into a single path
### Parameters

- `dir_names`: A list of directory names

### Returns
A string representing the concatenated path
*/
pub fn concat_path(dir_names: &[&str]) -> String {
    let mut path = String::new();
    for (idx, dir) in dir_names.iter().enumerate() {
        if idx != 0 {
            path.push('/');
        }
        path.push_str(dir);
    }
    return path;
}

pub enum FileType<'a> {
    RawStr(&'static str),
    String(&'a String),
    Number(&'a Number),
    Bool(&'a bool),
    Null(),
}

impl<'a> FileType<'a> {
    pub fn to_string(&self) -> String {
        match self {
            FileType::Null() => ("null").to_string(),
            FileType::Bool(val) => val.to_string(),
            FileType::Number(val) => val.to_string(),
            FileType::String(val) => val.to_string(),
            FileType::RawStr(val) => val.to_string(),
        }
    }
}

/**
**Create a file on a specific path**

### Parameters
- `directory`: path to the parent directory
- `file_name`: name of the file (checkout FileType for type options)
- `content`: optional field to write anything inside the file
*/
pub fn create_file<'a>(directory: &str, file_name: FileType<'a>, content: Option<&str>) {
    let mut file = File::create(&concat_path(&[directory, &file_name.to_string()]))
        .expect("Could not create file");

    if let Some(content) = content {
        file.write_all(content.as_bytes()).unwrap();
    }
}
