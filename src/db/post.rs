use diesel::{RunQueryDsl};
use diesel::{PgConnection};

use crate::schema::posts;
use crate::models::{Likes,Post,NewPost};
use crate::db::{Photo};

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
                .returning( posts::id )
                .get_result(db);

            let post_id = match result {
                    Ok(post_id) => post_id,
                    _ => return None,
            };
            match Likes::create_table(post_id,db) {
                Err(_) => return None,
                _ => return Some(post_id),
            }

        } else{
            return None;
        }

    }
}

enum PostErrors{
    InvalidPost,
    AlreadyLiked
}

impl Post {  
    fn from_id( post_id: i64 , db: &PgConnection) -> Option<Post> {
        let query = format!("SELECT * FROM posts WHERE id = \'{}\'",post_id);
        let a :Result<Vec<Post>,_> = diesel::sql_query(query).load::<Post>(db);
        match a {
            Ok(mut vec) => return vec.pop(),
            _ => return None,
        }
    }
   
    fn add_like( post_id: i64, user_id: i64, db: &PgConnection) -> Result<(),PostErrors>{
        if Likes::add_like(post_id,user_id,db) {
            Ok(())
        } else {
            Err(PostErrors::AlreadyLiked)
        }
    }
    fn get_who_likes( post_id: i64, db: &PgConnection) -> Result<Vec<String>,PostErrors>{
        unimplemented!();
    }

    fn delete( post_id: i64, db: &PgConnection ) {
        // posts::table.s
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
    let post_id = new_post.insert(&db).unwrap();
    println!("Post created with id: {}", post_id);

    let post = Post::from_id(post_id, &db).unwrap();
    println!("And it looks like this: {:?}", post);
    
    let like_count = Likes::count(post_id, &db);
    println!("It has this many likes: {}", like_count.unwrap());
    
    Likes::add_like(post_id, 1, &db);
    println!("It has this many likes: {}", Likes::count(post_id, &db).unwrap() );
    Likes::add_like(post_id, 3, &db);
    Likes::add_like(post_id, 3, &db);
    println!("It has this many likes: {}", Likes::count(post_id, &db).unwrap() );


}