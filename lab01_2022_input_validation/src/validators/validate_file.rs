use infer::{is_image, is_video, get, Type};

use crate::validators::error_messages::{INVALID_FILE_GROUP, INVALID_FILE_TYPE};
use crate::validators::file_helper::{read_from_path};

/// Validates if the file has the same extension that his file type
/// # Arguments
/// * `file_type` - The file type containing the extension
/// * `file_path` - The file path to validate
/// # Returns
/// * `bool` - True if the file has the same extension that his file type, false otherwise
fn match_extension(file_path: &str, type_file: &Type) -> bool {
    file_path.trim().to_lowercase().ends_with(type_file.extension())
}

/// Verify if the file has a valid extension in case it is a special extension
/// where a mime type can contain multiple extensions.
/// # Arguments
/// * `file_path` - The path of the file to be validated.
/// * `type_file` - The type of the file to be validated.
/// # Returns
/// `true` if the file has a valid special extension, `false` otherwise.
fn is_special_extension(file_path: &str, type_file: &Type) -> bool {
    // We accept special extension (tiff and jpeg) from ref:
    // jpg / jpeg and tif / tiff
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
    match type_file.extension() {
        "jpg" | "jpeg" => file_path.trim().to_lowercase().ends_with("jpeg"),
        "tif" | "tiff" => file_path.trim().to_lowercase().ends_with("tiff"),
        _ => false
    }
}

/// Check if the given file path owns the valid content and extension
/// # Arguments
/// * `file_path` - The file path to check
/// * `verify_extension` - True if the extension must be verified, false otherwise
/// # Returns
/// `bool` True - if the file has the valid content and extension, false otherwise
/// # Errors
/// * `&str` - An error message if the file isn't a video or an image or if an other error occurs
pub fn validate_file(file_path: &str, verify_extension: bool) -> Result<bool, &str> {
    let buffer = read_from_path(file_path)?;

    // Verify the group of the file type
    if !is_video(&buffer) && !is_image(&buffer) {
        return Err(INVALID_FILE_GROUP);
    }

    // Verify the extension of the file if asked
    let file_type_buffer = get(&buffer);
    match file_type_buffer {
        Some(file_type) => {
            Ok(!verify_extension
                || match_extension(file_path, &file_type)
                || is_special_extension(file_path, &file_type))
        },
        None => Err(INVALID_FILE_TYPE),
    }
}

// TODO : implement unit testing
#[cfg(test)]
mod tests {
    use crate::validators::validate_file::{validate_file};
    use crate::validators::test_helper::{result_helper};
    use crate::validators::error_messages::{INVALID_FILE_GROUP, ERROR_READING_FILE};

    // Tests has been written with file example found here:
    // https://file-examples.com/
    // Files are on my github but not delivered for the scholar rendering

    // Modified folder contains files that has a content x and has an extension y
    // such as a jpg file has an php extension
    static BASE_FILE_PATH : &str = "A:/HEIG/Semestre 6/SEC/Labos/SEC_Labo1/files/";
    static VIDEOS_FOLDER : &str = "videos/";
    static IMAGES_FOLDER : &str = "images/";
    static OTHERS_FOLDER: &str = "others/";
    static VIDEOS_MODIFIED_FOLDER : &str = "videos_modified/";
    static IMAGES_MODIFIED_FOLDER : &str = "images_modified/";
    static OTHERS_MODIFIED_FOLDER : &str = "others_modified/";
    static NAMING_CONVENTION : &str = "file_example_";

    #[test]
    fn validate_file_classical() {
        // Pass
        // Image
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, IMAGES_FOLDER, NAMING_CONVENTION, "png.png"),
            true), true, None);
        // Video
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, VIDEOS_FOLDER, NAMING_CONVENTION, "mov.mov"),
            true), true, None);

        // Fail
        // Other
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, OTHERS_FOLDER, NAMING_CONVENTION, "csv.csv"),
            true), false, Some(INVALID_FILE_GROUP));
    }

    #[test]
    fn validate_file_videos() {
        // Pass
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, VIDEOS_FOLDER, NAMING_CONVENTION, "avi.avi"),
            true), true, None);
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, VIDEOS_FOLDER, NAMING_CONVENTION, "mov.mov"),
            true), true, None);
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, VIDEOS_FOLDER, NAMING_CONVENTION, "mp4.mp4"),
            true), true, None);
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, VIDEOS_FOLDER, NAMING_CONVENTION, "webm.webm"),
            true), true, None);
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, VIDEOS_FOLDER, NAMING_CONVENTION, "wmv.wmv"),
            true), true, None);

        // Fail
        // Refused by lib infer as video
        // https://docs.rs/infer/0.7.0/infer/video/index.html
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, VIDEOS_FOLDER, NAMING_CONVENTION, "ogg.ogg"),
            true), false, Some(INVALID_FILE_GROUP));
    }

    #[test]
    fn validate_file_videos_modified() {
        // Corner cases & Fail
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, VIDEOS_MODIFIED_FOLDER, NAMING_CONVENTION, "avi.docx"),
            true), false, None);
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, VIDEOS_MODIFIED_FOLDER, NAMING_CONVENTION, "avi.mp4"),
            true), false, None);
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, VIDEOS_MODIFIED_FOLDER, NAMING_CONVENTION, "mov.xlsx"),
            true), false, None);
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, VIDEOS_MODIFIED_FOLDER, NAMING_CONVENTION, "mp4.avi"),
            true), false, None);
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, VIDEOS_MODIFIED_FOLDER, NAMING_CONVENTION, "webm.exe"),
            true), false, None);
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, VIDEOS_MODIFIED_FOLDER, NAMING_CONVENTION, "wmv.go"),
            true), false, None);
    }

    #[test]
    fn validate_file_images() {
        // Pass
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, IMAGES_FOLDER, NAMING_CONVENTION, "gif.gif"),
            true), true, None);
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, IMAGES_FOLDER, NAMING_CONVENTION, "ico.ico"),
            true), true, None);
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, IMAGES_FOLDER, NAMING_CONVENTION, "jpg.jpg"),
            true), true, None);
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, IMAGES_FOLDER, NAMING_CONVENTION, "png.png"),
            true), true, None);
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, IMAGES_FOLDER, NAMING_CONVENTION, "tiff.tif"),
            true), true, None);
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, IMAGES_FOLDER, NAMING_CONVENTION, "webp.webp"),
            true), true, None);

        // Fail
        // Refused by lib infer as image
        // https://docs.rs/infer/0.7.0/infer/image/index.html
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, IMAGES_FOLDER, NAMING_CONVENTION, "svg.svg"),
            true), false, Some(INVALID_FILE_GROUP));
    }

    #[test]
    fn validate_file_images_modified() {
        // Corner cases & Fail
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, IMAGES_MODIFIED_FOLDER, NAMING_CONVENTION, "gif.php"),
            true), false, None);
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, IMAGES_MODIFIED_FOLDER, NAMING_CONVENTION, "ico.c"),
            true), false, None);
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, IMAGES_MODIFIED_FOLDER, NAMING_CONVENTION, "jpg.java"),
            true), false, None);
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, IMAGES_MODIFIED_FOLDER, NAMING_CONVENTION, "png.jpg"),
            true), false, None);
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, IMAGES_MODIFIED_FOLDER, NAMING_CONVENTION, "tiff.png"),
            true), false, None);
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, IMAGES_MODIFIED_FOLDER, NAMING_CONVENTION, "webp.cpp"),
            true), false, None);
    }

    #[test]
    fn validate_file_special_images() {
        // Corner cases & Pass
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, IMAGES_FOLDER, NAMING_CONVENTION, "jpg.jpeg"),
            true), true, None);
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, IMAGES_FOLDER, NAMING_CONVENTION, "tiff.tiff"),
            true), true, None);
    }

    #[test]
    fn validate_file_others() {
        // Fail
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, OTHERS_FOLDER, NAMING_CONVENTION, "csv.csv"),
            true), false, Some(INVALID_FILE_GROUP));
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, OTHERS_FOLDER, NAMING_CONVENTION, "docx.docx"),
            true), false, Some(INVALID_FILE_GROUP));
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, OTHERS_FOLDER, NAMING_CONVENTION, "pptx.pptx"),
            true), false, Some(INVALID_FILE_GROUP));
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, OTHERS_FOLDER, NAMING_CONVENTION, "txt.txt"),
            true), false, Some(INVALID_FILE_GROUP));
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, OTHERS_FOLDER, NAMING_CONVENTION, "xlsx.xlsx"),
            true), false, Some(INVALID_FILE_GROUP));
    }

    #[test]
    fn validate_file_others_modified() {
        // Corner cases & Fail
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, OTHERS_MODIFIED_FOLDER, NAMING_CONVENTION, "csv.png"),
            true), false, Some(INVALID_FILE_GROUP));
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, OTHERS_MODIFIED_FOLDER, NAMING_CONVENTION, "docx.jpg"),
            true), false, Some(INVALID_FILE_GROUP));
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, OTHERS_MODIFIED_FOLDER, NAMING_CONVENTION, "pptx.mp4"),
            true), false, Some(INVALID_FILE_GROUP));
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, OTHERS_MODIFIED_FOLDER, NAMING_CONVENTION, "txt.mov"),
            true), false, Some(INVALID_FILE_GROUP));
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, OTHERS_MODIFIED_FOLDER, NAMING_CONVENTION, "xlsx.gif"),
            true), false, Some(INVALID_FILE_GROUP));
    }

    #[test]
    fn validate_file_without_extension_check() {
        // Corner Cases & Pass
        // Because we don't verify extension

        // Images
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, IMAGES_MODIFIED_FOLDER, NAMING_CONVENTION, "gif.php"),
            false), true, None);
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, IMAGES_MODIFIED_FOLDER, NAMING_CONVENTION, "tiff.png"),
            false), true, None);

        // Videos
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, VIDEOS_MODIFIED_FOLDER, NAMING_CONVENTION, "avi.mp4"),
            false), true, None);
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, VIDEOS_MODIFIED_FOLDER, NAMING_CONVENTION, "mov.xlsx"),
            false), true, None);

        // Corner Cases & Fail
        // Others
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, OTHERS_MODIFIED_FOLDER, NAMING_CONVENTION, "csv.png"),
            false), false, Some(INVALID_FILE_GROUP));
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, OTHERS_MODIFIED_FOLDER, NAMING_CONVENTION, "docx.jpg"),
            false), false, Some(INVALID_FILE_GROUP));
    }

    #[test]
    fn validate_file_no_file_found() {
        // Corner Cases & Fail
        result_helper(validate_file(
            &format!("{}{}{}{}", BASE_FILE_PATH, IMAGES_MODIFIED_FOLDER, NAMING_CONVENTION, "test.test"),
            false), false, Some(ERROR_READING_FILE));
    }
}
