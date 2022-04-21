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

// Message that are used more than once
static INVALID_FILE_CONTENT: &str = "Invalid file contents !";
static INVALID_UUID : &str = "Invalid UUID !";

/// struct used to store file datas and informations in map
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

/// This function is used to determine if the file is a video or an image
/// and then indicate the folder where the file will be stored
/// # Arguments
/// * `buffer` - The file buffer
/// # Return
/// * `(&str, &str)` - The folder where the file will be stored and the group file type
/// # Errors
/// * `&str` - An error message if the file is not a video or an image
pub fn get_file_storage_group_from_bytes(buffer: &Vec<u8>) -> Result<(&str, &str), &str> {
    if is_video(buffer) {
        Ok((STORAGE_VIDEOS_PATH, GROUP_VIDEO))
    } else if is_image(buffer) {
        Ok((STORAGE_IMAGES_PATH, GROUP_IMAGE))
    } else {
        Err(INVALID_FILE_GROUP)
    }
}

/// This function is used to generate a UUID for the file given
/// # Arguments
/// * `buffer` - The file buffer
/// # Return
/// * `String` - The UUID generated in hyphenated format
fn generate_uuid(buffer: &Vec<u8>) -> String {
    uuid::Uuid::new_v5(&Uuid::default(), &buffer)
        .to_hyphenated().to_string()
}

/// This function is used to store the file in the map with all the informations / data
/// it store the file if the file is valid and if the file is not already stored
/// # Arguments
/// * `file_path` - The file path given
/// # Return
/// * `String` - The success message
/// # Errors
/// * `String` - An error message if the file is not valid or if the file is already stored
/// or if an other error occurs
fn upload_file(file_path: &String) -> Result<String, String> {
    // Read file
    let buffer;
    match read_from_path(file_path) {
        Ok(buf) => buffer = buf,
        Err(error) => return Err(format!("{}", error)),
    };

    let file_valid;
    match validate_file(file_path, true) {
        Ok(valid) => file_valid = valid,
        Err(_) => return Err(INVALID_FILE_CONTENT.to_string()),
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
        Err(INVALID_FILE_CONTENT.to_string())
    }
}

/// This function handle the upload request by asking the user to give the file path
/// of the file to upload and then store it
/// if an error occurs, it will show it and ask the user to try again
fn file_upload_handler() {
    let mut uploaded = false;
    while !uploaded {
        let file_path = input::<String>()
            .msg("Please enter the path to an image or video file : ").get();

        match upload_file(&file_path) {
            Ok(message) => {
                println!("{}", message);
                uploaded = true;
            },
            Err(error) => println!("{}", error),
        }
    }
}

/// This function is used to verify if the file is uploaded or not
/// # Arguments
/// * `uuid` - The uuid of the file to verify
/// # Return
/// * `String` - The message indicating if the file is uploaded or not
fn verify_file_with_uuid(uuid: &String) -> String {
    let map = HASHMAP.lock().unwrap();
    if map.contains_key(uuid){
        let file_data = map.get(uuid).unwrap();
        format!("File {} exists, it's an {} file.", uuid, file_data.file_group)
    } else {
        format!("File not found with uuid: {}", uuid)
    }
}

/// This function handle the verify request by asking the user to give the uuid of the file
fn file_verify_handler() {
    let uuid = input::<String>()
        .msg("Please enter the UUID to check : ").get();

    if validate_uuid(&uuid) {
        println!("{}", verify_file_with_uuid(&uuid));
    } else {
        println!("{}", INVALID_UUID);
    }
}

/// This function is used to give the path of the file with the given uuid
/// # Arguments
/// * `uuid` - The uuid of the file to get the path
/// # Return
/// * `String` - The path of the file or an error message if the file doesn't exists
fn get_file_url_with_uuid(uuid: &str) -> String {
    let map = HASHMAP.lock().unwrap();
    if map.contains_key(uuid) {
        let file_data = map.get(uuid).unwrap();
        format!("{}{}", file_data.server_file_path, file_data.base_file_path)
    } else {
        format!("File not found with uuid: {}", uuid)
    }
}

/// This function handle the get url request by asking the user to give the uuid of the file
fn get_url_handler() {
    let uuid = input::<String>()
        .msg("Please enter the UUID to get : ").get();

    if validate_uuid(&uuid) {
        println!("{}", get_file_url_with_uuid(&uuid));
    } else {
        println!("{}", INVALID_UUID);
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
