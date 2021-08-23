pub mod photo;
pub mod user;
pub mod post;
pub mod likes;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use photo::Photo;

use crate::config::Config;
impl Config {
    pub fn sql_connection_string(&self) -> String {
        let usr = if self.database.passwd.is_empty() {
            format!("{}", self.database.user)
        } else {
            format!("{}:{}", self.database.user, self.database.passwd)
        };

        let url = format!("{}:{}", self.sqlserver.address, self.sqlserver.port);
        let database = self.database.name.clone();

        format!(
            "postgresql://{usr}@{url}/{db}",
            usr = usr,
            url = url,
            db = database
        )
    }
}

pub fn init( config: &Config ) -> PgConnection {
    Photo::init(config.photo_db.temp.as_str(), 
                config.photo_db.photo.as_str(), 
                config.photo_db.photo_timeout
            ).unwrap();

    
    PgConnection::establish( config.sql_connection_string().as_str() ).
            expect("Error connecting to DB"  )

}


#[test]
fn db_test(){
    let config = Config::default();
    init( &config);
}