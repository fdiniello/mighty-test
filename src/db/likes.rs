use diesel::{PgConnection, RunQueryDsl};
use crate::models::Likes;


impl Likes {
    pub fn create_table(post_id: i64, db: &PgConnection) -> Result<(), diesel::result::Error> {
        let query = format!("DROP TABLE IF EXISTS post{}_likes",post_id);
        diesel::sql_query(query).execute(db)?;
        
        let query = format!(
            "CREATE TABLE post{}_likes (
                id BIGINT UNIQUE PRIMARY KEY NOT NULL,
                CONSTRAINT fk_user
                FOREIGN KEY(id) 
                REFERENCES users(id)
            );",post_id);
            
        diesel::sql_query(query).execute(db)?;
    
        Ok(())
    }
    pub fn get_likes(post_id: i64, db: &PgConnection) -> Vec<Likes> {
        let query = format!("SELECT id FROM post{}_likes",post_id);
        let res : Result<Vec<Likes>,_> = diesel::sql_query(query).load::<Likes>(db);
        if let Ok(n) = res{
            n
        } else {
            vec![]
        }
    }
    pub fn count(post_id: i64, db: &PgConnection) -> Option<i64> {
        let query = format!("SELECT id FROM post{}_likes",post_id);
        
        let res : Result<Vec<Likes>,_> = diesel::sql_query(query).load::<Likes>(db);
        if let Ok(n) = res{
            Some( n.len() as i64 )
        } else {
            None
        }
    }
    pub fn add_like(post_id: i64, user_id: i64, db: &PgConnection) -> bool {
        let query = format!("INSERT INTO post{}_likes VALUES ({})", post_id, user_id);
        match diesel::sql_query(query).execute(db) {
            Ok(_) => true,
            _ => false,
        }
    }
}   

   