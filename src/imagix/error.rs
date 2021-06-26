use std::convert::From;
use std::io;

#[derive(Debug)]
pub enum ImageXError {
    UserInputError(String),
    FileIOError(String),
}

impl From<io::Error> for ImageXError {
    fn from(_error: io::Error) -> Self {
        ImageXError::FileIOError("Error in File I/O".to_string())
    }
}
