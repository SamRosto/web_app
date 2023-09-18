#![allow(warnings)]

use serde_json::Map;
use serde_json::value::Value;

use crate::to_do::ItemTypes;
use crate::to_do::structs::{done::Done, pending::Pending};
use crate::to_do::traits::{
    get::Get,
    delete::Delete,
    create::Create,
    edit::Edit,
};

fn process_pending(item: Pending, cmd: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match cmd.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "create" => item.create(&item.super_struct.title, &item.super_struct.status.stringify(), &mut state),
        "edit" => item.set_to_done(&item.super_struct.title, &mut state),
        _ => println!("command {cmd} not supported"),
    }
}

fn process_done(item: Done, cmd: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match cmd.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "delete" => item.delete(&item.super_struct.title, &mut state),
        "edit" => item.set_to_pending(&item.super_struct.title, &mut state),
        _ => println!("{cmd} not supported"),
    }
}

pub fn process_input(input: ItemTypes, cmd: String, state: &Map<String, Value>) {
    match input {
        ItemTypes::Done(val) => process_done(val, cmd, state),
        ItemTypes::Pending(val) => process_pending(val, cmd, state),
    }
}
