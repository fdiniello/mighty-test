use std::io::Cursor;
use std::path::{Path, PathBuf};

use diesel::pg::PgConnection;
use diesel::RunQueryDsl;

use diesel::r2d2::State;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use rocket::data::{self, FromData};
use rocket::http::hyper::StatusCode;
use rocket::http::{ContentType, Status};
use rocket::response::{content, status};
use rocket::Response;
use rocket::response::NamedFile;
use rocket::response::status::NotFound;



use crate::models::User;
use crate::db::{DbConn,photo::{self,Photo}};

#[route(POST, path = "/upload/by/<user_id>", data = "<photo>")]
pub fn new<'a>(user_id: i64, photo: Vec<u8>, db: DbConn) -> Response<'a> {
    let user = User::from_id(user_id, &db);

    let mut response = Response::build();
    if user.is_some() && user.unwrap().can_upload {
        match Photo::new(&photo) {
            Ok(photo_rul) => response.status(Status::Ok)
                                    .header(ContentType::Plain)
                                    .sized_body(Cursor::new(photo_rul.path)),
            Err(_) => response.status(Status::BadRequest),
        };
    } else {
        response.status(Status::Unauthorized);
    }
    response.finalize()
}

#[route(GET, path = "/<file..>")]
pub fn get(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let prefix = dotenv::var("PHOTO_DB").unwrap();
    let path = Path::new( &prefix.as_str() ).join(file);
    NamedFile::open(&path).map_err(|e| NotFound(e.to_string()))
}