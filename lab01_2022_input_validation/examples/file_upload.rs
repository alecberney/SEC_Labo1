use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

use read_input::prelude::*;
use lab01_2022_input_validation::*;

// Use the hashmap as follows:
// ```
// let map = HASHMAP.lock().unwrap();
// ```
/*lazy_static! {
    static ref HASHMAP: Mutex<HashMap</* <TO COMPLETE> */, /* <COMPLETE> */>> = Mutex::new(HashMap::new());
}*/

// todo: stocker uuid / path et redonner path quand uuid

// TODO: IMPLEMENT UPLOAD LOGIC
fn file_upload_handler() {
    /*
    You must not copy the le when uploaded, but simply store its UUID so that you can
    recover its le path from the UUID at a later time.
    • You will only allow for image and video les to be uploaded.
    • You must not allow uploaded les to be overwritten.
    */
}

// TODO: IMPLEMENT VERIFY LOGIC
fn file_verify_handler() {
    // The verification must be done based on the UUID, and not the file path.
}

// TODO: IMPLEMENT GET URL LOGIC
fn get_url_handler() {}

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
