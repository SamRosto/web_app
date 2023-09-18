use actix_web::{web, HttpResponse};
use serde_json::value::Value;
use serde_json::Map;

use crate::to_do::{to_do_factory, enums::TaskStatus};
use crate::json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems};
use crate::processes::process_input;
use crate::jwt::JwToken;
use crate::state::read_file;

pub async fn delete(item: web::Json<ToDoItem>, token: JwToken) -> HttpResponse {
    println!("Message in the token: {}", token.message);
    let state: Map<String, Value> = read_file("./state.json");
    let status: TaskStatus;
    
    match &state.get(&item.title) {
        Some(value) =>{ 
            status = TaskStatus::from_string(value.as_str().unwrap().to_string())
        },
        None => return HttpResponse::NotFound().json(format!("{} not in state", &item.title)),
    }
    // after getting status and title of the item, we can construct item and pass it through `process_input` with a `delete` command
    let existing_item = to_do_factory(item.title.as_str(), status.clone());
    process_input(existing_item, "delete".to_owned(), &state);
    HttpResponse::Ok().json(ToDoItems::get_state())
}
