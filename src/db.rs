use rocket::http::Status;
use std::ops::Deref;

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use rocket::request::{self, FromRequest, Outcome};
use rocket::{Request, State};

use diesel::pg::PgConnection;
use diesel::{prelude::*, r2d2};

pub mod like;
pub mod photo;
pub mod post;
pub mod user;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

pub fn init() -> DbPool {
    photo::init().unwrap();

    let database_url = dotenv::var("DATABASE_URL").expect("DB URL not set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    DbPool::new(manager).expect("Error connecting to DB")
}

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
        let pool = request.guard::<State<DbPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn db_test() {
    init();
}
