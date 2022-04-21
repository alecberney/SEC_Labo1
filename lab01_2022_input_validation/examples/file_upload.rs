use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use infer::{is_video, is_image};

use read_input::prelude::*;
use uuid::Uuid;
use lab01_2022_input_validation::*;

static STORAGE_IMAGES_PATH: &str = "sec.upload/images/";
static STORAGE_VIDEOS_PATH: &str = "sec.upload/videos/";
static GROUP_IMAGE: &str = "image";
static GROUP_VIDEO: &str = "video";

struct FileStorageData {
    base_file_path: String,
    server_file_path: String,
    file_group: String,
}

// Use the hashmap as follows:
// ```
// let map = HASHMAP.lock().unwrap();
// ```
lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, FileStorageData>> = Mutex::new(HashMap::new());
}

/// This function is used
pub fn get_file_storage_group_from_bytes(buffer: &Vec<u8>) -> Result<(&str, &str), &str> {
    if is_video(buffer) {
        Ok((STORAGE_VIDEOS_PATH, GROUP_VIDEO))
    } else if is_image(buffer) {
        Ok((STORAGE_IMAGES_PATH, GROUP_IMAGE))
    } else {
        Err(INVALID_FILE_GROUP)
    }
}

fn generate_uuid(buffer: &Vec<u8>) -> String {
    uuid::Uuid::new_v5(&Uuid::default(), &buffer)
        .to_hyphenated().to_string()
}

fn file_upload(file_path: &String) -> Result<String, String> {
    // Read file
    let buffer;
    match read_from_path(file_path) {
        Ok(buf) => buffer = buf,
        Err(error) => return Err(format!("{}", error)),
    };

    let file_valid;
    match validate_file(file_path, true) {
        Ok(valid) => file_valid = valid,
        Err(error) => return Err(format!("{}", error)),
    };

    if file_valid {
        let uuid = generate_uuid(&buffer);

        // Store uuid and path to map if uuid doesn't already exists
        let mut map = HASHMAP.lock().unwrap();
        if !map.contains_key(&uuid) {

            // Get infos
            let server_filepath;
            let group;
            match get_file_storage_group_from_bytes(&buffer) {
                Ok((path, grp)) => {
                    server_filepath = path;
                    group = grp;
                },
                Err(e) => return Err(e.to_string()),
            };

            map.insert(uuid.clone(), FileStorageData {
                base_file_path: file_path.to_string(),
                server_file_path : server_filepath.to_string(),
                file_group : group.to_string(),
            });

            Ok(format!("File uploaded successfully, UUID : {}", uuid))
        } else {
            Err(format!("File already exists with uuid: {}", uuid))
        }
    } else {
        Err("Invalid file contents !".to_string())
    }
}

fn file_upload_handler() {
    let mut uploaded = false;
    while !uploaded {
        let file_path = input::<String>()
            .msg("Please enter the path to an image or video file : ").get();

        match file_upload(&file_path) {
            Ok(message) => {
                println!("{}", message);
                uploaded = true;
            },
            Err(error) => println!("{}", error),
        }
    }
}

fn file_verify(uuid: &String) -> String {
    let map = HASHMAP.lock().unwrap();
    if map.contains_key(uuid){
        let file_data = map.get(uuid).unwrap();
        format!("File {} exists, it's an {} file.", uuid, file_data.file_group)
    } else {
        format!("File not found with uuid: {}", uuid)
    }
}


fn file_verify_handler() {
    let uuid = input::<String>()
        .msg("Please enter the UUID to check : ").get();

    if validate_uuid(&uuid) {
        println!("{}", file_verify(&uuid));
    } else {
        println!("Invalid UUID");
    }
}

fn get_url(uuid: &String) -> String {
    let map = HASHMAP.lock().unwrap();
    if map.contains_key(uuid) {
        let file_data = map.get(uuid).unwrap();
        format!("{}{}", file_data.server_file_path, file_data.base_file_path)
    } else {
        format!("File not found with uuid: {}", uuid)
    }
}

fn get_url_handler() {
    let uuid = input::<String>()
        .msg("Please enter the UUID to get : ").get();

    if validate_uuid(&uuid) {
        println!("{}", get_url(&uuid));
    } else {
        println!("Invalid UUID");
    }
}

fn main() {
    println!("Welcome to the super secure file upload tool !");
    loop {
        match input::<i32>().repeat_msg("Please select one of the following options to continue :\n1 - Upload a file\n2 - Verify file exists\n3 - Get file URL\n0 - Exit\nYour input ? [0-3]")
            .min_max(0, 3).get() {
            0 => {
                println!("Goodbye!");
                break
            },
            1 => file_upload_handler(),
            2 => file_verify_handler(),
            3 => get_url_handler(),
            _ => panic!("Invalid input"),
        }
    }
}
