use actix_web::Responder;
// use serde_json::value::Value;
// use serde_json::Map;

// use crate::state::read_file;
// use crate::to_do::{ItemTypes, to_do_factory, enums::TaskStatus};
use crate::json_serialization::to_do_items::ToDoItems;

pub async fn get() -> impl Responder {
    // let state: Map<String, Value> = read_file("./state.json");

    // let mut arr_buffer = Vec::new();

    // for (key, val) in state {
    //     let status = TaskStatus::from_string(&val.as_str().unwrap().to_string());
    //     let item: ItemTypes = to_do_factory(&key, status);
    //     arr_buffer.push(item);
    // }

    // let return_package: ToDoItems = ToDoItems::new(arr_buffer);

    // web::Json(return_package)
    ToDoItems::get_state()
}
