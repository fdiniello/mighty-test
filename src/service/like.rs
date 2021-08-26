use rocket::http::Status;
use rocket::Response;

use crate::db::DbConn;
use crate::models::Like;

#[route(POST, path = "/post/<post_id>/by/<user_id>")]
pub fn like_post<'r>(post_id: i64, user_id: i64, db: DbConn) -> rocket::Response<'r> {
    let like = Like { post_id, user_id };
    match like.insert(&db) {
        Ok(_) => Response::build().status(Status::Accepted).finalize(),
        _ => Response::build().status(Status::AlreadyReported).finalize(),
    }
}

#[route(GET, path = "/by/<user_id>")]
pub fn by_user(user_id: i64, db: DbConn) -> String {
    let like = Like {
        user_id,
        post_id: 0,
    };
    format!("{:?}", like.get_all(&db))
}

#[route(GET, path = "/for/<post_id>")]
pub fn for_post(post_id: i64, db: DbConn) -> String {
    let like = Like {
        post_id,
        user_id: 0,
    };
    format!("{:?}", like.get_all(&db))
}
