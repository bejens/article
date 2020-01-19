#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

mod api;
mod service;
mod dao;
mod entity;
mod client;
mod util;

fn main() {
    rocket::ignite().mount("/article", routes![api::article_api::create]).launch();
}