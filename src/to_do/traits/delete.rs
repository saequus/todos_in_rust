use crate::state::write_to_file;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;

pub trait Delete {
    fn delete(&self, title: &String, state: &mut Map<String, Value>) {
        state.remove(title);
        write_to_file("./state.json", state);
        println!("\n\n {} is being delete", title);
    }
}
