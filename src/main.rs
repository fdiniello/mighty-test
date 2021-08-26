#![allow(unused_imports)]
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

extern crate dotenv;

mod db;
mod models;
mod schema;
mod service;

fn main() {
    let s = service::build();
    s.launch();
}
