use serde::{Deserialize, Serialize};
use diesel::{ self, Queryable, Insertable,Identifiable};
use super::schema::posts;
use super::schema::users;

#[derive(Debug, Eq, PartialEq, Queryable, Identifiable)]
pub struct Post {
    pub id: i64,
    pub time_stamp: String,
    pub user_id: i64,
    pub file_path: String,
    pub comment: String,
    pub likes: Vec<i64>,
}


#[derive(Debug, Eq, PartialEq, Queryable)]
pub struct User {
    pub id: i64,
    pub user_id: i64,
    pub password: String,
    pub display_name: String,
    pub can_upload: bool,
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub user_id: i64,
    pub file_path: &'a str,
    pub comment: &'a str,
}

#[derive(Debug, Insertable)]
#[table_name="users"]
pub struct NewUser<'a>{
    pub user_name: &'a str,
    pub password: &'a str,
    pub display_name: &'a str,
}

