use infer::{is_image, is_video, get, Type};
//use infer::image::{is_avif, is_bmp, is_cr2, is_gif, is_heif, is_ico, is_jpeg, is_jpeg2000, is_jxr, is_png, is_psd, is_tiff, is_webp};
//use infer::video::{is_avi, is_flv, is_m4v, is_mkv, is_mov, is_mp4, is_mpeg, is_webm, is_wmv};

use crate::validators::error_messages::{INVALID_FILE_GROUP, INVALID_FILE_TYPE};
use crate::validators::file_helper::{read_from_path};

// Reference used: https://docs.rs/infer/0.7.0/infer/video/index.html
//static FILE_VIDEO_EXTENSION: [&str; 9]  = ["avi", "flv", "m4v", "mkv", "mov", "mp4", "mpeg", "webm", "wmv"];
// Reference used: https://docs.rs/infer/0.7.0/infer/image/index.html
//static FILE_IMAGE_EXTENSION: [&str; 13] = ["avif", "bmp", "cr2", "gif", "heif", "ico", "jpeg",
//"jpg2", "jxr", "png", "psd", "tiff", "webp"];
//static GROUP_TYPE: [&str; 2]  = ["video", "image"];

//static VIDEO_FILE_GROUP: &str = "video";
//static IMAGE_FILE_GROUP: &str = "image";

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types

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

fn match_extension(file_path: &str, type_file: &Type) -> bool {
    file_path.trim().to_lowercase().ends_with(type_file.extension())
}

/*fn is_video_or_image(type_file: &Type) -> bool {
    //type_file.matcher_type() == infer::MatcherType::Image || type_file.matcher_type() == infer::MatcherType::Video
    is_video(&buffer) || is_image(&buffer)
}*/

/// Check if the given file path owns the valid content and extension
/// # Arguments
/// * `file_path` - The file path to check
/// * `verify_extension` - True if the extension must be verified, false otherwise
/// # Returns
pub fn validate_file<'a>(file_path: &'a str, verify_extension: bool) -> Result<bool, &'a str> {

    let buffer = read_from_path(file_path)?;

    // https://docs.rs/infer/0.7.0/infer/fn.is_image.html
    // https://docs.rs/infer/0.7.0/infer/fn.is_video.html
    if !is_video(&buffer) && !is_image(&buffer) {
        return Err(INVALID_FILE_GROUP);
    }

    let file_type_buffer = get(&buffer);
    match file_type_buffer {
        Some(file_type) => {
            if verify_extension {
                Ok(match_extension(file_path, &file_type))
            } else {
                Ok(true)
            }
        },
        None => Err(INVALID_FILE_TYPE),
    }

    /*if file_path.to_lowercase().ends_with(file_type_buffer.extension())
        && file_type_buffer.mime_type() == file_mime_type
        && file_type_buffer.extension() == file_extension {
        Ok(true)
    } else {
        Ok(false)
    }*/
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
