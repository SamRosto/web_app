use serde_json::Map;
use serde_json::value::Value;

/// get todo item from a file
pub trait Get {
    fn get(&self, title: &String, state: &Map<String, Value>) {
        let item: Option<&Value> = state.get(title);
        match item {
            Some(status) => {
                println!("\nItem: {}", title);
                println!("Status: {}", status);
            },
            None => println!("item: {} not found", title)
        }
    }
} 
