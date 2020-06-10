use rocket_contrib::json::Json;
use crate::models::entry::Entry;

#[get("/entries")]
pub fn get_single_entry() -> Json<Entry> {
    Json(Entry::new(String::from("This is an entry on the database")))
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![get_single_entry]
}
