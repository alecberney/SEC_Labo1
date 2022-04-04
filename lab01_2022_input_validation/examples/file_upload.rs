use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

use read_input::prelude::*;
use lab01_2022_input_validation::*;

// Use the hashmap as follows:
// ```
// let map = HASHMAP.lock().unwrap();
// ```
lazy_static! {
    static ref HASHMAP: Mutex<HashMap</* <TO COMPLETE> */, /* <COMPLETE> */>> = Mutex::new(HashMap::new());
}

// TODO: IMPLEMENT UPLOAD LOGIC
fn file_upload_handler() {}

// TODO: IMPLEMENT VERIFY LOGIC
fn file_verify_handler() {}

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
