#![allow(warnings)]

use std::fs::read;

use actix_web::{web, HttpResponse};
use serde_json::value::Value;
use serde_json::Map;

use crate::state::read_file;
use crate::to_do::{to_do_factory, enums::TaskStatus};
use crate::json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems};
use crate::processes::process_input;

/// 1. Get the state of the entire app for the to-do items
/// 2. Check to see if the item is there, returning a `not found` response if it is not
/// 3. Pass the data through the `to_do_factory` factoey to construct the existing data from the state to an item that we can manipulate
/// 4. Check that the status being put in is not the same as the existing status
/// 5. Pass the existing item into the `process_input` function with `edit` command so it is saved to the JSON state file
/// 6. Get the state of the application and return it

pub async fn edit(item: web::Json<ToDoItem>) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json");
    let status: TaskStatus;

    match &state.get(&item.title) {
        Some(val) => {
            status = TaskStatus::from_string(val.as_str().unwrap().to_string());
        },

        None => return HttpResponse::NotFound().json(format!("{}not in state", &item.title)),
    }

    let existing_item = to_do_factory(item.title.as_str(), status.clone());

    if &status.stringify() == &TaskStatus::from_string(item.status.as_str().to_string()).stringify() {
        return HttpResponse::Ok().json(ToDoItems::get_state())
    }

    process_input(existing_item, "edit".to_owned(), &state);
    return HttpResponse::Ok().json(ToDoItems::get_state())
}