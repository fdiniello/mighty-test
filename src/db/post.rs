use std::process::id;

use diesel::{IntoSql, QueryDsl, RunQueryDsl, sql_types::Array, update};
use diesel::{PgConnection};

use crate::schema::posts;
use crate::models::{Post,NewPost};
use crate::db::{Photo, post};

impl<'a> NewPost<'a> {
    
    fn insert(&mut self, db: &PgConnection) -> Option<i64> {
        if let Ok(new_path) = Photo::realocate( &self.file_path ) {
            
            let corrected = NewPost{
                user_id: self.user_id,
                comment: self.comment,
                file_path: &new_path,
            };
            
            let result: Result<i64,_> = diesel::insert_into( posts::table )
                .values( &corrected )
                .returning( posts::id)
                .get_result(db);
            match result {
                    Ok(post_id) => return Some(post_id),
                    _ => return None,
            }
        } else{
            return None;
        }

    }
}

enum PostErrors{
    InvalidPost,
    AlreadyLike
}

impl Post {  
    fn from_id( post_id: i64 , db: &PgConnection) -> Option<Post> {
        unimplemented!();
    }
   
    fn add_like( post_id: i64, user_id: i64, db: &PgConnection) -> Result<(),PostErrors>{

        // let res :Result<Array<i64>,_> = posts::table.select( posts::likes)
        //     .filter( id.eq(post_id) )
        //     .get_result(db);
        unimplemented!();
    }
    fn get_likes( post_id: i64, db: &PgConnection) -> Result<Vec<i64>,PostErrors>{
        unimplemented!();
    }
}





#[test]
fn post_test(){
    use diesel::prelude::*;

    Photo::init("./data/tmp", "./data/photo", 300).unwrap();
    let database_url = crate::Config::default().sql_connection_string();
    let db = PgConnection::establish( database_url.as_str() ).unwrap();

    let photo = Photo::new(&[0; 12]).unwrap();

    let mut new_post = NewPost{
        user_id: 9,
        file_path: photo.get_path(),
        comment: "Doing bad at the bat cave",
    };
    let post = new_post.insert(&db);
    println!("Post created with id: {:?}", post );
    if let Some(post_id) = post {
        if let Ok(vec) = Post::get_likes(post_id, &db){
            println!("Got this much likes from {}: {:?}", post_id, vec);
        }
    }

}