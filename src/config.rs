#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::{env, fs, io::Write, net::SocketAddr};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Config {
    listen: Listen,
    pub sqlserver: SqlServer,
    pub database: DataBase,
    pub photo_db: PhotoDB,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Listen {
    address: String,
    port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SqlServer {
    pub address: String,
    pub port: u16,
    pub pool_size: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DataBase {
    pub user: String,
    pub passwd: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PhotoDB{
    pub temp: String,
    pub photo: String,
    pub photo_timeout: i32,
}


impl Config {
    pub fn default() -> Config {
        Config {
            listen: Listen::default(),
            sqlserver: SqlServer::default(),
            database: DataBase::default(),
            photo_db: PhotoDB::default(),
        }
    }

    pub fn init() -> Config {
        let args: Vec<String> = env::args().collect();

        let path = if args.len() > 1 {
            args[1].as_str()
        } else {
            "Config.toml"
        };

        if let Ok(config) = Config::read(path) {
            println!("Succesfully loaded {}", path);
            config
        } else {
            println!("Unable to load {}, using default config", path);
            Config::default()
        }
    }
    pub fn listen_address(&self) -> SocketAddr {
        let address = format!("{}:{}", self.listen.address, self.listen.port);
        let sa: SocketAddr = address
            .as_str()
            .parse()
            .expect("Unable to parse socket address");
        sa
    }

    pub fn read(path: &str) -> Result<Config, toml::de::Error> {
        let file = fs::read_to_string(path).unwrap_or(String::from(""));
        let config = toml::from_str(file.as_str());
        config
    }

    pub fn write(self: &Self, path: &'static str) {
        let toml = toml::to_string(&self).unwrap();
        let mut file = std::fs::File::create(path).expect("create failed");
        write!(&mut file, "{}", toml).unwrap();
    }
}

impl SqlServer {
    fn default() -> SqlServer {
        SqlServer {
            address: "localhost".to_string(),
            port: 5432,
            pool_size: 15,
        }
    }
}
impl DataBase {
    fn default() -> DataBase {
        DataBase {
            user: "postgres".to_string(),
            passwd: "CAXw6zWg8inQ8A".to_string(),
            name: "mighty_test".to_string(),
        }
    }
}

impl Listen {
    pub fn default() -> Listen{
        Listen{
            address: "127.0.0.1".to_string(),
            port: 3030,
        }
    }
}

impl PhotoDB {
    pub fn default() -> PhotoDB {
        PhotoDB{
            temp: "./data/tmp".to_owned(),
            photo: "./data/photo_storage".to_owned(),
            photo_timeout: 300,
        }
    }
}

#[test]
fn config_test() {
    let default = Config::default();
    default.write("default.toml");
    let test = Config::read("default.toml").unwrap();
    
    assert_eq!(default, test);
    
}
