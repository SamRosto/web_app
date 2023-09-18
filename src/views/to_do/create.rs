
// 1. Load current state of the to-do item list
// 2. Get the title of the new to-do item from th eURL
// 3. Pass the title and the status `pending` through `to_do_factory`
// 4. Pass the result of the previous step along with the string `create` and the state into the
//    process module interface
// 5. Return a string to the user to signal that the process has finished
//
// `serde_json::value::Value` and `serde::Map` are used to define what type of data we are reading
//    from state.json file. To extract the title from URL we are using `HttpRequest` struct.
// We will then import what we need from othrt modules to enable us to create an item, read the
// state file, and process the input

use serde_json::value::Value;
use serde_json::Map;

use actix_web::HttpResponse;
use actix_web::HttpRequest;

use crate::to_do::{to_do_factory, enums::TaskStatus};
use crate::json_serialization::to_do_items::ToDoItems;
use crate::state::read_file;
use crate::processes::process_input;

pub async fn create(req: HttpRequest) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json"); // step 1
    let title = req.match_info().get("title").unwrap().to_string(); // step 2
    let item = to_do_factory(&title.as_str(), TaskStatus::PENDING); // step 3
    process_input(item, "create".to_string(), &state);
    HttpResponse::Ok().json(ToDoItems::get_state())
}
