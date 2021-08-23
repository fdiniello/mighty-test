use std::io::Write;
use std::thread;
use std::time::{SystemTime,Duration};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use std::fs::{self, File, metadata};

static mut TEMP_DB :String = String::new();
static mut PHOTO_DB : String = String::new();

#[derive(Debug, Clone)]
pub struct Photo{
    path: String,
}

impl Photo{
    pub fn new( data: &[u8]) -> std::io::Result<Photo> {
        let temp = unsafe {
            TEMP_DB.as_str()
        };

        let temp_path = temp.to_string() + "/" + random_string(15).as_str() + ".png";

        let mut file = File::create(temp_path.clone())?;
        file.write(&data)?;
        file.flush()?;

        Ok(Photo { path: temp_path })
    }
    pub fn from_str(s: &str) -> Photo {
        Photo { path: String::from(s) }
    }
    pub fn get_path(&self) -> &str{
        self.path.as_str()
    }
    pub fn realocate( src: &str ) -> std::io::Result<String>{
        let db = unsafe {
            PHOTO_DB.as_str()
        };

        // Define a new path for the photo to be stored in, under the main photo_db path;
        let mut new_path = db.to_string() + "/" + random_string(3).as_str() + "/" + random_string(3).as_str();
        // Create the new dirs in case the don't exist
        fs::create_dir_all( new_path.clone() )?;

        new_path += "/";
        new_path += random_string(15).as_str();
        new_path += ".png";

        fs::rename(src, &new_path )?;

        
        Ok(String::from(new_path))
    }

    pub fn rm( path: &str ){
        fs::remove_file( path );
    }

    pub fn init(temp: &str, photo: &str, photo_timeout: i32) -> std::io::Result<()> {
        unsafe{
            TEMP_DB = temp.to_string();
            PHOTO_DB = photo.to_string();
        }
        fs::create_dir_all(temp.clone() )?;
        fs::create_dir_all(photo )?;
        let temp_dir = temp.to_string();
        thread::spawn( move ||{
            Photo::cleanup_loop( temp_dir, photo_timeout);
        });

        Ok(())
    }

    fn cleanup_loop( temp_dir : String, photo_timeout: i32 ){
        loop {
            thread::sleep( Duration::new( (photo_timeout/4) as u64, 0) );
            println!("Cleaning up photos that didn't complete the proper post creation in the last {} seconds.", photo_timeout/4);

            let mut rm_list = Vec::new();
            let current_time = SystemTime::now();

            if let Ok(files) = fs::read_dir( &temp_dir ){
                    for entry in  files {
                         match entry {
                             Ok( valid_entry) => {
                                if let Ok(metadata) = metadata(valid_entry.path()) {
                                    if let Ok(creation) = metadata.modified() {
                                        if let Ok( elpased ) = current_time.duration_since(creation) {
                                            if elpased.as_secs() > photo_timeout as u64 {
                                                rm_list.push(valid_entry.path());
                                            }
                                        }
                                    }
                                }
                             },
                             _ =>{},
                         }
                    }

            } 
            
            if !rm_list.is_empty() {
                println!("Found {} files to delete", rm_list.len() );
                for path in rm_list  {
                    let _ = fs::remove_file( path );
                }
            }
        }
    }
}

fn random_string(len: usize) -> String{
    thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
}

#[test]
fn new_realocate_test() {
    let tmp: &str = "./data/tmp";
    let photo: &str = "./data/photo_storage";
    Photo::init(tmp, photo, 300).unwrap();
    
    let mut data = [0; 12];
    data.clone_from_slice(b"hola mundo!\n");
    
    let a = Photo::new(&data).unwrap();
    println!( "Created photo with path: {:?}", a );
    
    let a = Photo::realocate( a.get_path() ).unwrap();
    println!( "Relocated photo with path: {:?}", a );
}

#[test]
fn cleanup_test(){
    let tmp: &str = "./data/tmp";
    let photo: &str = "./data/photo_storage";
    Photo::init(tmp, photo, 5).unwrap();


    for _ in 1..10 {
        thread::sleep( Duration::new( 2, 0) );
        
        let data = [0; 12];
        let a = Photo::new(&data).unwrap();
        println!( "Created photo with path: {:?}", a );
    }
    thread::sleep( Duration::new(10,0 ));


}