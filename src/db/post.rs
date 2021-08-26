use std::env;

use diesel::RunQueryDsl;

use crate::db::photo::{self, Photo};
use crate::models::{Like, NewPost, NewUser, Post, User};
use crate::schema::posts;
use diesel::pg::PgConnection;

impl<'a> NewPost<'a> {
    pub fn insert(&self, db: &PgConnection) -> Option<i64> {
        if let Ok(new_path) = Photo::realocate(&self.file_path) {
            let corrected = NewPost {
                user_id: self.user_id,
                comment: self.comment,
                file_path: &new_path,
            };

            let result: Result<i64, _> = diesel::insert_into(posts::table)
                .values(&corrected)
                .returning(posts::id)
                .get_result(db);

            match result {
                Ok(post_id) => return Some(post_id),
                _ => return None,
            };
        } else {
            return None;
        }
    }
}

impl Post {
    pub fn from_id(post_id: i64, db: &PgConnection) -> Option<Post> {
        let query = format!("SELECT * FROM posts WHERE id = \'{}\'", post_id);
        let a: Result<Vec<Post>, _> = diesel::sql_query(query).load::<Post>(db);
        match a {
            Ok(mut vec) => return vec.pop(),
            _ => return None,
        }
    }

    pub fn delete(post_id: i64, db: &PgConnection) -> Result<(), ()> {
        let post = Post::from_id(post_id, &db);
        if post.is_some() {
            Photo::rm(&post.unwrap().file_path);
            let like = Like {
                post_id,
                user_id: 0,
            };
            match like.clear_likes(db) {
                Ok(_) => {}
                _ => return Err(()),
            };
            let query = format!("DELETE FROM Posts WHERE post_id = {}", post_id);
            match diesel::sql_query(query).execute(db) {
                Ok(_) => Ok(()),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}

#[test]
fn post_test() {
    use diesel::prelude::*;

    photo::init().unwrap();
    let db = crate::db::init().get().unwrap();

    let photo = Photo::new(&[0; 12]).unwrap();
    let user_id = NewUser {
        user_name: "Batman4ever",
        display_name: "Bruce Wayne",
        password: "",
    }
    .insert(&db)
    .unwrap();
    let new_post = NewPost {
        user_id,
        file_path: photo.get_path(),
        comment: "Something something",
    };
    let post_id = new_post.insert(&db).unwrap();
    let post = Post::from_id(post_id, &db).unwrap();

    assert_eq!(post.comment, new_post.comment);
    assert_eq!(post.user_id, new_post.user_id);
    assert_ne!(post.file_path, new_post.file_path);

    Post::delete(post_id, &db);
    assert!(Post::from_id(post_id, &db).is_some());
    User::delete(user_id, &db);
}
