use regex::Regex;
use std::io::{Error, ErrorKind};

/// Validate a file by checking that it is an image or a video. And check his filename extension
/// if requested.
///
/// The result is an unsigned integer telling:
///     0 - The file is invalid (not a video or image or the extension is invalid)
///     1 - The file is valid and it is an image
///     2 - The file is valid and it a video
///
/// # Errors
/// If the filename could not be found or opened. Also return an error if the file type is unknown
/// (cf. crate infer).
///
/// # Examples
/// ``` ignore
/// match validate_file("myDir/myImage.png", true) {
///     Ok(result) => match result {
///         0 => println!("Invalid file contents !"),
///         1 => println!("The file is valid and it is an image !"),
///         2 => println!("The file is valid and it is a video !"),
///         _ => {},
///     },
///     Err(e) => println!("An error occurred: {}", e.to_string()),
/// }
/// ```
pub fn validate_file(filename: &str, check_extension: bool) -> Result<u8, Error> {
    // Read the file to check the magic numbers
    match infer::get_from_path(filename)? {
        None => Err(Error::new(ErrorKind::Other, "File type is unknown.")),

        Some(kind) => {
            // Check the extension if requested
            if check_extension {
                // Case is irrelevant for the extension
                let file_extension = kind.extension().to_lowercase();
                let regex = Regex::new(&format!(r"{}$", file_extension)).unwrap();
                if !regex.is_match(&filename.to_lowercase()) {
                    return Ok(0);
                }
            }

            // Check if the file is an image (1), a video (2) or other (0)
            match kind.matcher_type() {
                infer::MatcherType::Image => Ok(1),
                infer::MatcherType::Video => Ok(2),
                _ => Ok(0),
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::validate_file;

    const TEST_DIR: &str = "test_files";

    #[test]
    fn valid_files_image() {
        assert_eq!(validate_file(&format!("{}/valid_image.jpg", TEST_DIR), false).unwrap(), 1);
        assert_eq!(validate_file(&format!("{}/valid_image.png", TEST_DIR), false).unwrap(), 1);

        // valid image even if the extension doesn't correspond
        assert_eq!(validate_file(&format!("{}/invalid_ext_image_jpg.png", TEST_DIR), false).unwrap(), 1);
    }

    #[test]
    fn valid_files_video() {
        assert_eq!(validate_file(&format!("{}/valid_video.avi", TEST_DIR), false).unwrap(), 2);
        assert_eq!(validate_file(&format!("{}/valid_video.mov", TEST_DIR), false).unwrap(), 2);

        // valid video even if the extension doesn't correspond
        assert_eq!(validate_file(&format!("{}/invalid_ext_video_avi.mp4", TEST_DIR), false).unwrap(), 2);
    }

    #[test]
    fn invalid_files() {
        assert_eq!(validate_file(&format!("{}/invalid_file.pdf", TEST_DIR), false).unwrap(), 0);
        assert_eq!(validate_file(&format!("{}/invalid_file.ppt", TEST_DIR), false).unwrap(), 0);
    }

    #[test]
    fn valid_extensions() {
        assert_eq!(validate_file(&format!("{}/valid_image.jpg", TEST_DIR), true).unwrap(), 1);
        assert_eq!(validate_file(&format!("{}/valid_video.avi", TEST_DIR), true).unwrap(), 2);

        // extensions should not be case sensitive
        assert_eq!(validate_file(&format!("{}/valid_ext_image.JpG", TEST_DIR), true).unwrap(), 1);
        assert_eq!(validate_file(&format!("{}/valid_ext_video.AVI", TEST_DIR), true).unwrap(), 2);
    }

    #[test]
    fn invalid_extensions() {
        // content doesn't match extension
        assert_eq!(validate_file(&format!("{}/invalid_ext_image_jpg.png", TEST_DIR), true).unwrap(), 0);
        assert_eq!(validate_file(&format!("{}/invalid_ext_video_avi.mp4", TEST_DIR), true).unwrap(), 0);

        // adding extension doesn't pass
        assert_eq!(validate_file(&format!("{}/invalid_ext_image.jpg.png", TEST_DIR), true).unwrap(), 0);
        assert_eq!(validate_file(&format!("{}/invalid_ext_video.avi.mov", TEST_DIR), true).unwrap(), 0);
    }

    #[test]
    fn invalid_filepath() {
        assert_eq!(validate_file("", false).unwrap_err().to_string(), "No such file or directory (os error 2)");
        assert_eq!(validate_file(&format!("{}/oe.png", TEST_DIR), false).unwrap_err().to_string(), "No such file or directory (os error 2)");
    }

    #[test]
    fn invalid_file_type() {
        assert_eq!(validate_file("Cargo.toml", false).unwrap_err().to_string(), "File type is unknown.");
    }
}
