use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    content: String,
}

impl Entry {
    pub fn new(content: String) -> Entry {
        Entry {
            content,
        }
    }
}
