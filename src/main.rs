mod state;
mod to_do;

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
    #[derive(Debug)]
    enum CharacterValue {
        Name(String),
        Age(i32),
        Items(Vec<String>),
    }

    let mut profile: HashMap<&str, CharacterValue> = HashMap::new();

    profile.insert("name", CharacterValue::Name("Maxwell".to_string()));
    profile.insert("age", CharacterValue::Age(30));
    profile.insert("items", CharacterValue::Items(vec!["hat".to_string()]));

    match profile.get("name") {
        Some(value_data) => match value_data {
            CharacterValue::Name(name) => {
                println!("correct {:?}", name);
            }
            _ => panic!("not a string"),
        },
        None => {
            println!("missing ")
        }
    }

    match profile.get("name").unwrap() {
        CharacterValue::Name(name) => {
            println!("correct");
        }
        _ => panic!("error"),
    }

    println!("profile: {:?}", profile);

    fn is_error(check: bool) -> Result<i8, &'static str> {
        if check {
            Err("this is error")
        } else {
            Ok(1)
        }
    }

    #[derive(Debug)]
    enum Friend {
        HUMAN(Box<Human>),
        NIL,
    }

    #[derive(Debug)]
    struct Human {
        name: String,
        age: i8,
        thought: Option<String>,
        friend: Friend,
    }

    impl Human {
        fn new(name: &str, age: i8) -> Human {
            return Human {
                name: name.to_string(),
                age: age,
                thought: None,
                friend: Friend::NIL,
            };
        }

        fn with_thought(mut self, thought: &str) -> Human {
            self.thought = Some(thought.to_string());
            return self;
        }

        fn with_friend(mut self, friend: Box<Human>) -> Human {
            self.friend = Friend::HUMAN(friend);
            return self;
        }
    }

    let another_developer = Human::new("Tracy", 19);

    let developer = Human::new("Maxwell", 18)
        .with_thought("test t")
        .with_friend(Box::new(another_developer));

    trait CanEdit {
        fn edit(&self) {
            println!("editing")
        }
    }

    impl CanEdit for User {}

    struct User {
        email: String,
        password: String,
    }

    fn edit<T: CanEdit>(user: &T) -> () {
        user.edit()
    }

    let myuser = User {
        email: "temp".to_string(),
        password: "temp".to_string(),
    };

    edit(&myuser);

    // println!("{}", TaskStatus::DONE);
    // let output1 = TaskStatus::DONE.to_string();
    // println!("{}", output1);

    // println!("{}", TaskStatus::PENDING);
    // let output2 = TaskStatus::PENDING.to_string();
    // println!("{}", output2);

    // let to_do_item = to_do_factory("washing", TaskStatus::DONE);

    // match to_do_item {
    //     ItemTypes::Done(item) => {
    //         item.get(&item.super_struct.title);
    //         item.delete(&item.super_struct.title);
    //     },
    //     ItemTypes::Pending(item) => {
    //         item.get(&item.super_struct.title);
    //         item.set_to_done(&item.super_struct.title);
    //     }
    // }

    let args: Vec<String> = env::args().collect();
    let status: &String = &args[1];
    let title: &String = &args[2];
    let mut state: Map<String, Value> = read_file("./state.json");
    println!("Before operation: {:?}", state);
    state.insert(title.to_string(), json!(status));
    println!("After operation: {:?}", state);
    write_to_file("./state.json", &mut state);
}
