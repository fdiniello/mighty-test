use diesel::prelude::*;
use rocket::Rocket;
use std::net::SocketAddr;

use crate::db;

mod like;
mod post;
mod photo;

pub fn build() -> Rocket {
    rocket::ignite()
        .manage(db::init())
        .mount("/post", routes![post::single])
        .mount("/post", routes![post::get_page])
        .mount("/post", routes![post::new])
        .mount("/like", routes![like::like_post])
        .mount("/like", routes![like::for_post])
        .mount("/like", routes![like::by_user])
        .mount("/photo", routes![photo::new])
        .mount("/photo", routes![photo::get])
}
