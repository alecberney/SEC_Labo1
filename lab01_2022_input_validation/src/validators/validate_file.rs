use std::fs;
use lazy_static::lazy_static;
use regex::Regex;
use infer::{is_image, is_video, get};
//use infer::image::{is_avif, is_bmp, is_cr2, is_gif, is_heif, is_ico, is_jpeg, is_jpeg2000, is_jxr, is_png, is_psd, is_tiff, is_webp};
//use infer::video::{is_avi, is_flv, is_m4v, is_mkv, is_mov, is_mp4, is_mpeg, is_webm, is_wmv};

use crate::validators::error_messages::{ERROR_READING_FILE, INVALID_FILE_PATH, INVALID_FILE_GROUP, INVALID_FILE_TYPE};

static REGEX_FILE_PATH: &str = r"[a-zA-Z0-9\/\.\\_-]+";
// Reference used: https://docs.rs/infer/0.7.0/infer/video/index.html
static FILE_VIDEO_EXTENSION: [&str; 9]  = ["avi", "flv", "m4v", "mkv", "mov", "mp4", "mpeg", "webm", "wmv"];
// Reference used: https://docs.rs/infer/0.7.0/infer/image/index.html
static FILE_IMAGE_EXTENSION: [&str; 13] = ["avif", "bmp", "cr2", "gif", "heif", "ico", "jpeg",
"jpg2", "jxr", "png", "psd", "tiff", "webp"];
static GROUP_TYPE: [&str; 2]  = ["video", "image"];

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types

/// Check if the given file path is valid
/// # Arguments
/// * `file_path` - The file_path to check
/// # Returns
/// * `bool` - True if the file_path is valid, false otherwise
pub fn is_valid_file_path(file_path: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(&format!("^{}$", REGEX_FILE_PATH)).unwrap();
    }
    RE.is_match(file_path)
}

/// Check if the given file path is valid
/// # Arguments
/// * `file_group` - The file_group to check
/// # Returns
/// * `bool` - True if the file_group is valid, false otherwise
/*fn is_valid_file_group(file_group: &str) -> bool {
    let file_group_modified = &file_group.trim().to_lowercase();
    match file_group_modified {
        "video" => true,
        "image" => true,
        _ => false,
    }
}

fn is_given_file_type_right<'a>(buffer: &'a Vec<u8>, file_type: &'a str, file_group: &'a str) -> Result<bool, &'a str> {
    if !is_valid_file_group(file_group) {
        return Err(INVALID_FILE_GROUP);
    }

    let file_type_modified = &file_type.trim().to_lowercase();
    let file_group_modified = &file_group.trim().to_lowercase();
     //if is_valid_file_type(file_type) {
        if file_group_modified == "video" && is_video(buffer) {
            match file_type_modified {
                    "avi" => return Ok(is_avi(&buffer)),
                    "flv" => return Ok(is_flv(&buffer)),
                    "m4v" => return Ok(is_m4v(&buffer)),
                    "mkv" => return Ok(is_mkv(&buffer)),
                    "mov" => return Ok(is_mov(&buffer)),
                    "mp4" => return Ok(is_mp4(&buffer)),
                    "mpeg" => return Ok(is_mpeg(&buffer)),
                    "webm" => return Ok(is_webm(&buffer)),
                    "wmv" => return Ok(is_wmv(&buffer)),
                    _ => return Err(INVALID_FILE_TYPE),
            }
        } else if file_group_modified == "image" && is_image(buffer) {
            match file_type_modified {
                    "avif" => return Ok(is_avif(&buffer)),
                    "bmp" => return Ok(is_bmp(&buffer)),
                    "cr2" => return Ok(is_cr2(&buffer)),
                    "gif" => return Ok(is_gif(&buffer)),
                    "heif" => return Ok(is_heif(&buffer)),
                    "ico" => return Ok(is_ico(&buffer)),
                    "jpeg" => return Ok(is_jpeg(&buffer)),
                    "jpg2" => return Ok(is_jpeg2000(&buffer)),
                    "jxr" => return Ok(is_jxr(&buffer)),
                    "png" => return Ok(is_png(&buffer)),
                    "psd" => return Ok(is_psd(&buffer)),
                    "tiff" => return Ok(is_tiff(&buffer)),
                    "webp" => return Ok(is_webp(&buffer)),
                    _ => return Err(INVALID_FILE_TYPE),
            }
        } else {
            return Err(INVALID_FILE_GROUP);
        }
    /*} else {
        Err(INVALID_FILE_TYPE)
    }*/
}*/

/// given file type or group must be checked

/*
You are required to validate that the le contents at a surface level match a given le type
or group (image or video) by using information such as magic bytes and headers. To do so,
you are allowed to use the infer library to extract mime type and extension information.
You should also provide the option to validate that the le extension matches the provided
lename, irrelevant of text case. You can reject all les that are not videos or images.
If needed, you can obtain sample image and video les from https://file-examples.
com/
*/

// todo passer extension ou mime type?
// todo doit passer le type souhaité?
pub fn validate_file<'a>(file_path: &'a str, file_type: &'a str, group_type: &'a str) -> Result<bool, &'a str> {
    // TODO: validation d'input

    if !is_valid_file_path(file_path) {
        return Err(INVALID_FILE_PATH);
    }

    // https://docs.rs/infer/0.7.0/infer/index.html
    // https://docs.rs/infer/0.7.0/infer/fn.get.html

    //fs::read()
    /*let kind = infer::get(buffer).expect("file type is known");

    assert_eq!(kind.mime_type(), "image/jpeg");
    assert_eq!(kind.extension(), "jpg");
    assert_eq!(kind.matcher_type(), infer::MatcherType::Image);*/

    // Read file
    let buffer = fs::read(file_path);
    match buffer {
        Ok(buffer) => buffer,
        Err(_) => return Err(ERROR_READING_FILE),
    };

    let info = infer::Infer::new();
    //let buf = [0xFF, 0xD8, 0xFF, 0xAA];
    let file_type_buffer = info.get(&buffer).expect("file type is known");

    if !is_video(&buffer) && !is_image(&buffer) {
        return Err(INVALID_FILE_GROUP);
    }

    // https://docs.rs/infer/0.7.0/infer/fn.is_image.html
    // https://docs.rs/infer/0.7.0/infer/fn.is_video.html
    if file_type_buffer.mime_type() == file_type && file_type_buffer.extension() == file_type {
        Ok(true)
    } else {
        Ok(false)
    }
}

// TODO : implement unit testing
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    //https://file-examples.com/
}
