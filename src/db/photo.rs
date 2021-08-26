use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs::{self, metadata, File};
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use std::{env, thread};

use crate::db::photo;

#[derive(Debug, Clone)]
pub struct Photo {
   pub path: String,
}

impl Photo {
    pub fn new(data: &[u8]) -> std::io::Result<Photo> {
        let temp_dir = dotenv::var("PHOTO_TMP_DIR").unwrap();
        let photo_path : String  = random_string(15) + ".png";

        let full_path = temp_dir + "/" + photo_path.as_str().clone();
        let mut file = File::create(full_path )?;
        file.write(&data)?;
        file.flush()?;

        Ok(Photo { path: photo_path })
    }
    pub fn from_str(s: &str) -> Photo {
        Photo {
            path: String::from(s),
        }
    }
    pub fn get_path(&self) -> &str {
        self.path.as_str()
    }
    pub fn realocate(src: &str) -> std::io::Result<String> {
        let db_dir = dotenv::var("PHOTO_DB").unwrap();
        let temp_dir = dotenv::var("PHOTO_TMP_DIR").unwrap();

        // Define a new path for the photo to be stored in, under the main photo_db path;
        let folder : String = [ random_string(3),
                                "/".to_owned(),
                                random_string(3)].concat();

        // Create the new dirs in case the don't exist
        fs::create_dir_all([ &db_dir, "/", &folder ].concat() )?;

        let new_path : String = [ folder, 
                                    "/".to_owned(),
                                    random_string(15),
                                    ".png".to_owned() ].concat();

        let original = temp_dir + "/" + src;
        let destination = db_dir + "/" + new_path.clone().as_str();
        fs::rename(original, destination)?;
        Ok(String::from(new_path))
    }

    pub fn rm(path: &str) {
        fs::remove_file(path);
    }
}

fn random_string(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn init() -> std::io::Result<()> {
    let temp = dotenv::var("PHOTO_TMP_DIR").expect("Temporary directory for photos not set");
    let photo = dotenv::var("PHOTO_DB").expect("Directory for photos not set");
    let photo_timeout = dotenv::var("PHOTO_TIMEOUT")
        .unwrap()
        .parse::<i32>()
        .expect("Timeout for temp photos not set");

    fs::create_dir_all(temp.clone())?;
    fs::create_dir_all(photo)?;
    let temp_dir = temp.to_string();
    thread::spawn(move || {
        cleanup_loop(temp_dir, photo_timeout);
    });

    Ok(())
}
fn get_files(temp_dir: &String, photo_timeout: i32) -> Vec<PathBuf> {
    let current_time = SystemTime::now();
    let mut list = Vec::new();

    if let Ok(files) = fs::read_dir(&temp_dir) {
        for entry in files {
            match entry {
                Ok(valid_entry) => {
                    if let Ok(metadata) = metadata(valid_entry.path()) {
                        if let Ok(creation) = metadata.modified() {
                            if let Ok(elpased) = current_time.duration_since(creation) {
                                if elpased.as_secs() > photo_timeout as u64 {
                                    list.push(valid_entry.path());
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    list
}
fn cleanup_loop(temp_dir: String, photo_timeout: i32) {
    loop {
        thread::sleep(Duration::new((photo_timeout / 4) as u64, 0));
        let rm_list = get_files(&temp_dir, photo_timeout);

        if !rm_list.is_empty() {
            println!("Found {} files to delete", rm_list.len());
            for path in rm_list {
                let _ = fs::remove_file(path);
            }
        }
    }
}

#[test]
fn new_realocate_test() {
    init().unwrap();

    let mut data = [0; 12];
    data.clone_from_slice(b"hola mundo!\n");

    let a = Photo::new(&data).unwrap();
    println!("Created photo with path: {:?}", a);

    let b = Photo::realocate(a.get_path()).unwrap();
    println!("Relocated photo with path: {:?}", b);
    assert_ne!(a.path, b);
}

#[test]
fn cleanup_test() {
    init().unwrap();

    for _ in 1..10 {
        thread::sleep(Duration::new(2, 0));

        let data = [0; 12];
        let a = Photo::new(&data).unwrap();
        println!("Created photo with path: {:?}", a);
    }
    let photo_timeout = dotenv::var("PHOTO_TIMEOUT")
        .unwrap()
        .parse::<u64>()
        .expect("Timeout for temp photos not set");

    thread::sleep(Duration::new(photo_timeout / 4, 0));

    let files = get_files(&dotenv::var("PHOTO_TMP_DIR").unwrap(), 0);
    assert_eq!(files.len(), 0)
}
