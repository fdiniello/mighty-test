use diesel::pg::PgConnection;
use diesel::RunQueryDsl;

use diesel::r2d2::State;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use rocket::http::hyper::StatusCode;
use rocket::http::{ContentType, Status};
use rocket::response::{content, status};
use rocket::Response;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::db::DbConn;
use crate::models::{Like, NewPost, Post, User};

#[derive(Debug, Serialize)]
pub struct PostwLikes {
    id: i64,
    time_stamp: std::time::SystemTime,
    user_id: i64,
    file_path: String,
    comment: String,
    likes: usize,
}
impl<'a> PostwLikes {
    fn from(p: Post, db: &DbConn) -> PostwLikes {
        PostwLikes {
            id: p.id,
            time_stamp: p.time_stamp,
            user_id: p.user_id,
            file_path: p.file_path,
            comment: p.comment,
            likes: Like {
                post_id: p.id,
                user_id: 0,
            }
            .get_all(&db)
            .len(),
        }
    }
}

#[route(GET, path = "/get/<id>")]
pub fn single<'a>(id: i64, db: DbConn) -> Json<PostwLikes> {
    let p = Post::from_id(id, &db).unwrap();

    Json(PostwLikes::from(p, &db))
}

#[route(GET, path = "/get/page/<nth>/size/<size>")]
pub fn get_page(nth: i64, size: i64, db: DbConn) -> Json<Vec<PostwLikes>> {
    let query = format!(
        "SELECT * FROM Posts ORDER BY time_stamp DESC OFFSET {} ROWS FETCH NEXT {} ROWS ONLY",
        nth * size,
        size
    );
    let res: Result<Vec<Post>, _> = diesel::sql_query(query).load::<Post>(&db as &PgConnection);

    let mut post_with_likes = vec![];
    match res {
        Err(_) => {}
        Ok(posts) => {
            for p in posts {
                post_with_likes.push(PostwLikes::from(p, &db))
            }
        }
    };
    Json(post_with_likes)
}

#[route(POST, path = "/new", format = "json", data = "<post>")]
pub fn new<'a>(post: Json<NewPost>, db: DbConn) -> Response<'a> {
    let user = User::from_id(post.user_id, &db);

    let mut response = Response::build();
    if user.is_some() && user.unwrap().can_upload {
        match post.insert(&db) {
            Some(_) => response.status(Status::Ok),
            None => response.status(Status::BadRequest),
        };
    } else {
        response.status(Status::Unauthorized);
    }
    response.finalize()
}
