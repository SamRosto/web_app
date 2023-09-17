use serde_json::value::Value;
use serde_json::Map;

use crate::state::write_file;

/// Deletes item in to do list
pub trait Delete{
    fn delete(&self, title: &String, state: &mut Map<String,Value>) {
        state.remove(title);
        write_file("./state.json", state);
        println!("\n{title} is deleted");
    }
}
