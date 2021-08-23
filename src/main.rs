#![allow(unused_imports)]
#[macro_use]
extern crate diesel;


mod config;
mod models;
mod schema;
mod db;

use config::Config;

fn main() {
    let config = Config::init();
    
    let db = db::init(&config);
    
}