use crate::db::DbConn;
use crate::models::Like;
use crate::schema::likes;
use diesel::{result::Error, PgConnection, RunQueryDsl};

pub enum LikeError {
    AlreadyLiked,
}

impl Like {
    pub fn insert(self, db: &PgConnection) -> Result<(), LikeError> {
        match diesel::insert_into(likes::table).values(&self).execute(db) {
            Ok(_) => Ok(()),
            Err(_) => Err(LikeError::AlreadyLiked),
        }
    }
    pub fn clear_likes(&self, db: &PgConnection) -> Result<(), ()> {
        let query = format!("DELETE FROM Likes WHERE post_id = {}", self.post_id);
        match diesel::sql_query(query).execute(db) {
            Ok(_) => Ok(()),
            _ => Err(()),
        }
    }
    pub fn get_all(&self, db: &PgConnection) -> Vec<Like> {
        let query = if self.post_id != 0 {
            format!("SELECT * FROM Likes WHERE post_id = {}", self.post_id)
        } else {
            format!("SELECT * FROM Likes WHERE user_id = {}", self.user_id)
        };

        let res: Result<Vec<Like>, _> = diesel::sql_query(query).load::<Like>(db);

        if let Ok(n) = res {
            n
        } else {
            vec![]
        }
    }
    pub fn count(&self, db: &PgConnection) -> i64 {
        let n = self.get_all(db);
        n.len() as i64
    }
}
