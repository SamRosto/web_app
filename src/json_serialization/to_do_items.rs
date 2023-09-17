/// Define what is returned to the viewer when `get` view is called 
/// in the current file

use serde::{Serialize};
use serde_json::value::Value;
use serde_json::Map;

use actix_web:: {
    body::BoxBody,
    http::header::ContentType,
    HttpRequest, HttpResponse, Responder
};

use crate::to_do::ItemTypes;
use crate::to_do::structs::base::Base;
use crate::state::read_file;
use crate::to_do::{to_do_factory, enums::TaskStatus};


#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_cnt: i8,
    pub done_item_cnt: i8,
}

impl ToDoItems {
    pub fn new(input: Vec<ItemTypes>) -> ToDoItems {
        let mut pending_arr_buffer = Vec::new();
        let mut done_arr_buffer = Vec::new();

        for item in input {
            match item {
                ItemTypes::Pending(item) => pending_arr_buffer.push(item.super_struct),
                ItemTypes::Done(item) => done_arr_buffer.push(item.super_struct),
            }
        }

        let pending_cnt = pending_arr_buffer.len() as i8;
        let done_cnt = done_arr_buffer.len() as i8;

        ToDoItems { 
            pending_items: pending_arr_buffer,
            done_items: done_arr_buffer,
            pending_item_cnt: pending_cnt,
            done_item_cnt: done_cnt,
        }
    }

    pub fn get_state() -> ToDoItems {
        let state: Map<String, Value> = read_file("./state.json");
        let mut arr_buffer = Vec::new();

        for (key, value) in state {
            let status = TaskStatus::from_string(&value.as_str().unwrap().to_string());
            let item = to_do_factory(&key, status);
            arr_buffer.push(item);
        }

        ToDoItems::new(arr_buffer)
    }
}

/// Serialize `ToDoItems` struct using the `serde_json` crate, and return HTTP response with the `ToDoItems` at the body
impl Responder for ToDoItems {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok().content_type(ContentType::json()).body(body)
    }
}