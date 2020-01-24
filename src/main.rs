#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

mod api;
mod client;
mod dao;
mod entity;
mod service;
mod util;

fn main() {
    rocket::ignite()
        .mount("/article", routes![api::article_api::create])
        .launch();
}
