use thiserror::Error;

/// Client Errors
#[derive(Debug, Error)]
pub enum Error {
    /// Imgur API returned non-successful status code
    #[error("server reponse non-successful status code - {0}, body = `{1}`")]
    ApiError(u16, String),
    /// Imgur API returned non-successful status code (body is too long)
    #[error("server reponse non-successful status code - {0}, (response body is too long)")]
    ApiErrorBodyTooLong(u16),
    /// Invalid file path or URL adress
    #[error("{0} is not url or file path")]
    InvalidUrlOrFile(String),
    /// reqwest::Error
    #[error("reqwest error - {0}")]
    ReqwestError(reqwest::Error),
    /// std::io::Error
    #[error("io error - {0}")]
    IoError(std::io::Error),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::ReqwestError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

/// A `Result` alias where the `Err` case is `imgurs::Error`
pub type Result<T> = std::result::Result<T, Error>;
