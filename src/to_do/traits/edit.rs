use crate::state::write_to_file;
use crate::to_do::enums::TaskStatus;
use serde_json::{json, Map, Value};

pub trait Edit {
    fn set_to_done(&self, title: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(TaskStatus::DONE.stringify()));
        write_to_file("./state.json", state);
        println!("\n\n {} is set to done", title.to_string());
    }

    fn set_to_pending(&self, title: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(TaskStatus::PENDING.stringify()));
        write_to_file("./state.json", state);
        println!("\n\n {} is set to pending", title.to_string());
    }
}
