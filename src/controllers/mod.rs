pub mod user;
pub mod entry;

pub fn get_routes() -> Vec<rocket::Route> {
    let mut routes = Vec::new();
    let mut user_routes = user::get_routes();
    let mut entry_routes = entry::get_routes();

    routes.append(&mut user_routes);
    routes.append(&mut entry_routes);

    routes
}
