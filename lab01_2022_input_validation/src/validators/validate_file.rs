use std::fmt::format;
use std::fs;
use lazy_static::lazy_static;
use regex::Regex;
use infer::{is_image, is_video};
use infer::image::{is_avif, is_bmp, is_cr2, is_gif, is_heif, is_ico, is_jpeg, is_jpeg2000, is_jxr, is_png, is_psd, is_tiff, is_webp};
use infer::video::{is_avi, is_flv, is_m4v, is_mkv, is_mov, is_mp4, is_mpeg, is_webm, is_wmv};

use crate::validators::error_messages;
use crate::validators::error_messages::{ERROR_READING_FILE, INVALID_FILE_PATH, INVALID_FILE_GROUP, INVALID_FILE_TYPE};

static REGEX_FILE_PATH: &str = r"[a-zA-Z0-9\/\.\\_-]+";
// Reference used: https://docs.rs/infer/0.7.0/infer/video/index.html
static FILE_VIDEO_EXTENSION: Vec<&str>  = ["AVI", "FLV", "M4V", "MKV", "MOV", "MP4", "MPEG", "WEBM", "WMV"];
// Reference used: https://docs.rs/infer/0.7.0/infer/image/index.html
static FILE_IMAGE_EXTENSION: Vec<&str> = ["AVIF", "BMP", "CR2", "GIF", "HEIF", "ICO", "JPEG", "JPG2", "JXR", "PNG", "PSD", "TIFF", "WEBP"];
static GROUP_TYPE: Vec<&str>  = ["video", "image"];

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
    file_group = file_group.trim().to_lowercase();
    match file_group {
        "video" => true,
        "image" => true,
        _ => false,
    }
}

fn is_given_file_type_right(buffer: &Vec<u8>, file_type: &str, file_group: &str) -> Result<bool, &str> {
    if !is_valid_file_group(file_group) {
        Err(INVALID_FILE_GROUP)
    }

     if is_valid_file_type(file_type) {
        if file_group == "video" && is_video(buffer) {
            match file_type {
                    "AVI" => return Ok(is_avi(&buffer)),
                    "FLV" => return Ok(is_flv(&buffer)),
                    "M4V" => return Ok(is_m4v(&buffer)),
                    "MKV" => return Ok(is_mkv(&buffer)),
                    "MOV" => return Ok(is_mov(&buffer)),
                    "MP4" => return Ok(is_mp4(&buffer)),
                    "MPEG" => return Ok(is_mpeg(&buffer)),
                    "WEBM" => return Ok(is_webm(&buffer)),
                    "WMV" => return Ok(is_wmv(&buffer)),
                    _ => return Err(INVALID_FILE_TYPE),
            }
        } else if file_group == "image" && is_image(buffer) {
            match file_type {
                    "AVIF" => return Ok(is_avif(&buffer)),
                    "BMP" => return Ok(is_bmp(&buffer)),
                    "CR2" => return Ok(is_cr2(&buffer)),
                    "GIF" => return Ok(is_gif(&buffer)),
                    "HEIF" => return Ok(is_heif(&buffer)),
                    "ICO" => return Ok(is_ico(&buffer)),
                    "JPEG" => return Ok(is_jpeg(&buffer)),
                    "JPG2" => return Ok(is_jpeg2000(&buffer)),
                    "JXR" => return Ok(is_jxr(&buffer)),
                    "PNG" => return Ok(is_png(&buffer)),
                    "PSD" => return Ok(is_psd(&buffer)),
                    "TdIFF" => return Ok(is_tiff(&buffer)),
                    "WEBP" => return Ok(is_webp(&buffer)),
                    _ => return Err(INVALID_FILE_TYPE),
            }
        } else {
            return Err(INVALID_FILE_GROUP);
        }
    } else {
        Err(INVALID_FILE_TYPE)
    }
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
pub fn validate_file(file_path: &str, file_type: &str, group_type: &str, must_match_filename: bool) -> Result<bool, &str> {
    // TODO: validation d'input

    if !is_valid_file_path(file_path) {
        Err(INVALID_FILE_PATH)
    }

    // https://docs.rs/infer/0.7.0/infer/index.html

    //fs::read()
    /*let kind = infer::get(buffer).expect("file type is known");

    assert_eq!(kind.mime_type(), "image/jpeg");
    assert_eq!(kind.extension(), "jpg");
    assert_eq!(kind.matcher_type(), infer::MatcherType::Image);*/

    // Read file
    let buffer = match &fs::read(file_path) {
        Ok(buffer) => buffer,
        Err(e) => {
            Err(ERROR_READING_FILE)
        }
    };

        let info = infer::Infer::new();
        //let buf = [0xFF, 0xD8, 0xFF, 0xAA];
        let file_type_buffer = info.get(&buffer).expect("file type is known");

       if file_type_buffer.mime_type() == "image/jpeg" && file_type_buffer.extension() == "jpg" {
            Ok(true)
       }

    // type.extension
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
