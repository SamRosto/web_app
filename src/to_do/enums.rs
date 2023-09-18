#[allow(warnings)]

use serde::ser::{Serialize, Serializer, SerializeStruct};

#[derive(Debug, Clone)]
pub enum TaskStatus {
    DONE,
    PENDING,
}

/// String format representation of task status
impl TaskStatus {
    pub fn stringify(&self) -> String {
        match &self {
            &Self::DONE => "DONE".to_string(),
            &Self::PENDING => "PENDING".to_string(),
        }
    }

    pub fn from_string(status: String) -> Self {
        match status.as_str() {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("{status} not supported"),
        }
    }
}

/// serialize fn gets called when serializing TaskStatus enum
impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        Ok(serializer.serialize_str(&self.stringify().as_str())?)
    }
}


