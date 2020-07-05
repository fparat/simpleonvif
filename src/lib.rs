#[macro_use]
extern crate log;

mod auth;
mod camera;
mod error;
mod namespaces;
mod onvif;

pub use camera::OnvifCamera;
pub use error::{Error, Result};
