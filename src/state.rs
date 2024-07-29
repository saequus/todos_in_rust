use std::fs;
use std::fs::File;
use std::io::Read;

use serde_json;
use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut file = File::open(file_name.to_string()).expect("Failed to open file");

    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Failed to read file to string");

    if data.trim().is_empty() {
        println!("Warning: The file is empty or contains only whitespace.");
        return Map::new();
    }

    let json: Value = serde_json::from_str(&data).expect("Failed to parse JSON");
    let state: Map<String, Value> = json.as_object().unwrap().clone();
    return state;
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    let new_data = json!(state);
    fs::write(file_name.to_string(), new_data.to_string()).expect("Unable to write file");
}
