use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
    email: String,
}

impl User {
    pub fn new(name: String, email: String) -> User {
        User {
            name,
            email,
        }
    }
}
