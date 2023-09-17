use super::base::Base;
use super::super::enums::TaskStatus;
use super::super::traits::{
    get::Get,
    edit::Edit,
    create::Create,
};

#[derive(Debug)]
pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(title: &str) -> Self {
        let base = Base {
            title: title.to_string(),
            status: TaskStatus::PENDING,
        };

        Pending { super_struct: base }
    }
}

impl Get for Pending {}
impl Edit for Pending {}
impl Create for Pending {}

