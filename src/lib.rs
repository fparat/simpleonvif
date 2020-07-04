#[macro_use]
extern crate log;

mod auth;
mod camera;
mod error;
pub mod namespaces;
pub mod onvif;

pub use camera::OnvifCamera;
pub use error::{Error, Result};
