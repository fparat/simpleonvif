use thiserror::Error;
use ureq::Response;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0:?}")]
    Response(Response),
    #[error("missing profile token")]
    MissingProfile,
}
