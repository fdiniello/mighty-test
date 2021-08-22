use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use std::net::SocketAddr;

fn hello_world()->String{
    format!("Hello World!")
}

pub fn run( listen_address: SocketAddr, _sql_pool: r2d2::Pool<PostgresConnectionManager<NoTls>> ){
    // let routes = build_routes();
}