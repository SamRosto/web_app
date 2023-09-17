#![allow(warnings)]
/// Define what is returned to the viewer when `get` view is called 
/// in the current file

use serde::Serialize;
use crate::to_do::ItemTypes;
use crate::to_do::structs::base::Base;

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
}