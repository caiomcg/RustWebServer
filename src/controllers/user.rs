use rocket_contrib::json::Json;
use crate::models::user::User;

#[get("/users")]
pub fn get_single_user() -> Json<User> {
    Json(User::new(String::from("Caio"), String::from("caiomcg@gmail.com")))
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![get_single_user]
}
