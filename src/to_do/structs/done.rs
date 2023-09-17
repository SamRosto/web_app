#![allow(warnings)]


use super::base::Base;
use super::super::enums::TaskStatus;
use super::super::traits::{
    get::Get,
    create::Create,
    delete::Delete,
    edit::Edit};

#[derive(Debug)]
pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(title: &str) -> Self {
        let base = Base {
            title: title.to_string(),
            status: TaskStatus::DONE,
        };

        Done { super_struct: base }
    }
}

impl Get for Done {}
impl Delete for Done {}
impl Edit for Done {}
