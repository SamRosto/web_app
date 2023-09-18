use serde::Deserialize;

/// Args:
///
/// title: String
/// 
/// status: String
#[derive(Deserialize)]
pub struct ToDoItem {
    pub title: String,
    pub status: String,
}