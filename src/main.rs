mod state;
mod to_do;
mod processes;

use std::collections::HashMap;
use std::env;
use std::fmt;
use to_do::enums::TaskStatus;
use to_do::to_do_factory;
use to_do::ItemTypes;

use crate::to_do::traits::delete::Delete;
use crate::to_do::traits::edit::Edit;
use crate::to_do::traits::get::Get;
use serde_json::value::Value;
use serde_json::{json, Map};
use state::{read_file, write_to_file};
use processes::process_input;

macro_rules! capitalize {
    ($a: expr) => {{
        let mut v: Vec<char> = $a.chars().collect();
        if let Some(first) = v.get_mut(0) {
            *first = first.to_uppercase().nth(0).unwrap();
        }
        v.into_iter().collect::<String>()
    }};
}

macro_rules! capitalize2 {
    (&a: expr) => {
        let mut v: Vec<char> = &a.chars().collect();
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        &a = v.into_iter().collect();
    };
}

macro_rules! capitalize3 {
    (&a: expr) => {{
        let mut v: Vec<char> = &a.chars().collect();
        if let Some(first) = v.get(0) {
            *first = first.to_uppercase().nth(0).unwrap();
        }
        v.into_iter().collect::<String>()
    }};
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];

    let state: Map<String, Value> = read_file("./state.json");
    let status: String;

    match &state.get(*&title) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        }
        None => {
            status = "pending".to_owned();
        }
    }

    let item = to_do_factory(
        title,
        TaskStatus::from_string(status.to_uppercase())
    );

    process_input(item, command.to_string(), &state)
    
}
