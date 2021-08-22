

use diesel::{IntoSql,RunQueryDsl};

use crate::models::{NewUser};
use crate::schema::users;

#[test]
fn populate_fake_users() {
    use diesel::{Connection, PgConnection};
    
    let database_url = crate::Config::default().sql_connection_string();
    let db = PgConnection::establish( database_url.as_str() ).unwrap();

    let fake_users = vec![
        NewUser{user_name: "Batman", password: "xRabikPsn3CWVA==", display_name: "Bruce Wayne" },
        NewUser{user_name: "CaptainAmerica", password: "ycg+9a3r+hOvgQ==", display_name: "Steve Rogers" },
        NewUser{user_name: "Daredevil", password: "580SlKOIEcEOag==", display_name: "Matt Murdock" },
        NewUser{user_name: "GreenArrow", password: "LpYQN65Zh84Qmg==", display_name: "Oliver Queen" },
        NewUser{user_name: "HeMan", password: "ns4sEZlWTXx+Zw==", display_name: "Prince Adam" },
        NewUser{user_name: "TheHulk", password: "2ddDROMZYGowAA==", display_name: "Bruce Banner" },
        NewUser{user_name: "JudgeDredd", password: "ss7caUtVVMI8LA==", display_name: "Joe Dredd" },
        NewUser{user_name: "Spiderman", password: "eNXMaPGOLf8wCA==", display_name: "Peter Parker" },
        NewUser{user_name: "Superman", password: "aHAQEOX7S/ipag==", display_name: "Clark Kent" },
        NewUser{user_name: "WonderWoman", password: "lTZ7OxJmYqGdqA==", display_name: "Diana Prince" },
    ];

    diesel::insert_into(users::table)
        .values(&fake_users)
        .execute(&db).unwrap();

}