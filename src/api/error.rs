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
    /// Imgur API error or reqwest::Error
    #[error("send request to imgur api: {0}")]
    SendApiRequest(reqwest::Error),
    /// std::io::Error
    #[error("io error - {0}")]
    IoError(std::io::Error),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::SendApiRequest(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

/// A `Result` alias where the `Err` case is `imgurs::Error`
pub type Result<T> = std::result::Result<T, Error>;
