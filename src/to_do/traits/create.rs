use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

use crate::state::write_file;

/// Creates new item
pub trait Create {
    fn create(&self, title: &String, status: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(status));
        write_file("./state.json", state);
        println!("{title} is created");
    }
} 
