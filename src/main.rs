#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod controllers;
mod models;

fn main() {
    rocket::ignite().mount("/api", controllers::get_routes()).launch();
}
