use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

use crate::state::{write_file};
use super::super::TaskStatus::{DONE, PENDING};

/// Trait to edit to do item
pub trait Edit {
    fn set_to_done(&self, title: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(DONE.stringify()));
        write_file("./state.json", state);
        println!("\n{title} is set to done");
    }

    fn set_to_pending(&self, title: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(PENDING.stringify()));
        write_file("./state.json", state);
        println!("\n{title} set to pending");
    }
}
