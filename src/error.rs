use thiserror::Error;
use ureq::Response;

/// Crate result type
pub type Result<T> = std::result::Result<T, Error>;

/// Crate error type
#[derive(Error, Debug)]
pub enum Error {
    #[error("{0:?}")]
    Response(Response),
    #[error("missing profile token")]
    MissingProfile,
    #[error("invalid URL")]
    InvalidUrl(#[from] url::ParseError),
}
