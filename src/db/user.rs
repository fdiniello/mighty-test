use std::env;

use crate::models::{NewUser, User};
use crate::schema::users;
use diesel::pg::PgConnection;
use diesel::{IntoSql, RunQueryDsl};

impl User {
    pub fn from_id(user_id: i64, db: &PgConnection) -> Option<User> {
        let query = format!("SELECT * FROM users WHERE id = \'{}\'", user_id);
        let a: Result<Vec<User>, _> = diesel::sql_query(query).load::<User>(db);
        match a {
            Ok(mut vec) => return vec.pop(),
            _ => return None,
        }
    }
    pub fn delete(user_id: i64, db: &PgConnection) {
        let query = format!("DELETE FROM Likes WHERE user_id = {}", user_id);
        diesel::sql_query(query).execute(db);
        let query = format!("DELETE FROM Posts WHERE user_id = {}", user_id);
        diesel::sql_query(query).execute(db);
        let query = format!("DELETE FROM Users WHERE id = {}", user_id);
        diesel::sql_query(query).execute(db);
    }
}

impl<'a> NewUser<'a> {
    pub fn insert(&self, db: &PgConnection) -> Option<i64> {
        let result: Result<i64, _> = diesel::insert_into(users::table)
            .values(&*self)
            .returning(users::id)
            .get_result(db);
        match result {
            Ok(id) => Some(id),
            _ => None,
        }
    }
}

#[test]
fn populate_fake_users() {
    use diesel::{Connection, PgConnection};
    let database_url = dotenv::var("DATABASE_URL").expect("DB URL not set");
    let db = PgConnection::establish(database_url.as_str()).unwrap();

    let fake_users = vec![
        NewUser {
            user_name: "Batman",
            password: "xRabikPsn3CWVA==",
            display_name: "Bruce Wayne",
        },
        NewUser {
            user_name: "CaptainAmerica",
            password: "ycg+9a3r+hOvgQ==",
            display_name: "Steve Rogers",
        },
        NewUser {
            user_name: "Daredevil",
            password: "580SlKOIEcEOag==",
            display_name: "Matt Murdock",
        },
        NewUser {
            user_name: "GreenArrow",
            password: "LpYQN65Zh84Qmg==",
            display_name: "Oliver Queen",
        },
        NewUser {
            user_name: "HeMan",
            password: "ns4sEZlWTXx+Zw==",
            display_name: "Prince Adam",
        },
        NewUser {
            user_name: "TheHulk",
            password: "2ddDROMZYGowAA==",
            display_name: "Bruce Banner",
        },
        NewUser {
            user_name: "JudgeDredd",
            password: "ss7caUtVVMI8LA==",
            display_name: "Joe Dredd",
        },
        NewUser {
            user_name: "Spiderman",
            password: "eNXMaPGOLf8wCA==",
            display_name: "Peter Parker",
        },
        NewUser {
            user_name: "Superman",
            password: "aHAQEOX7S/ipag==",
            display_name: "Clark Kent",
        },
        NewUser {
            user_name: "WonderWoman",
            password: "lTZ7OxJmYqGdqA==",
            display_name: "Diana Prince",
        },
    ];

    diesel::insert_into(users::table)
        .values(&fake_users)
        .execute(&db)
        .unwrap();
}
