use std::thread::sleep;
use std::time::Duration;

use regex::Regex;
use ureq::Response;

use crate::auth;
use crate::error::Result;
use crate::{onvif, Error};

/// Camera device
#[derive(Debug)]
pub struct OnvifCamera {
    address: String,
    profile: Option<String>,
    user: Option<String>,
    password: Option<String>,
}

impl OnvifCamera {
    /// Returns a new camera object. Note that the profile requires to be Some() for many methods.
    pub fn new(address: &str, profile: Option<&str>) -> Result<Self> {
        let u = url::Url::parse(address)?;

        let user = match u.username() {
            "" => None,
            name => Some(name.to_string()),
        };
        let password = u.password().map(String::from);

        Ok(Self {
            address: address.to_string(),
            profile: profile.map(String::from),
            user,
            password,
        })
    }

    /// Set the ONVIF user name
    pub fn with_user<S: Into<String>>(mut self, user: Option<S>) -> Self {
        self.user = user.map(|u| u.into());
        self
    }

    /// Set the ONVIF password
    pub fn with_password<S: Into<String>>(mut self, password: Option<S>) -> Self {
        self.password = password.map(|p| p.into());
        self
    }

    /// Return a new SOAP header element for authentication. Do not reuse it
    /// because the current time is used to generate the element.
    fn get_auth(&self) -> Option<String> {
        match (&self.user, &self.password) {
            (Some(ref u), Some(ref p)) => Some(auth::get_auth_header(u, p)),
            _ => None,
        }
    }

    /// Get the profile token
    // Return a Result so that we can use "?" in other methods
    fn profile_token(&self) -> Result<&str> {
        self.profile
            .as_ref()
            .map(|s| s.as_str())
            .ok_or(Error::MissingProfile)
    }

    /// Send the message with HTTP POST
    fn post(&self, msg: &str) -> Result<Response> {
        trace!("sending message = {}", &msg);
        let resp = ureq::post(&self.address).send_string(&msg);

        if resp.ok() {
            trace!("response ok");
            Ok(resp)
        } else {
            error!("response error {:?}", &resp);
            Err(Error::Response(resp))
        }
    }

    /// Fetch the available profiles from the camera
    pub fn get_profiles(&self) -> Result<Vec<String>> {
        trace!("get_profiles");

        let soap_msg =
            onvif::soap_envelop(onvif::soap_body(onvif::get_profiles()), self.get_auth());

        let resp = self.post(&soap_msg)?;

        // regex for parsing xml ohlala...
        let resp_str = resp.into_string().expect("resp is not a string?");
        let re = Regex::new(r#"<trt:Profiles .*?token="(.+?)">"#).unwrap();
        let profiles = re
            .captures_iter(&resp_str)
            .map(|cap| cap[1].to_string())
            .collect();

        Ok(profiles)
    }

    /// Order a continuous move to the camera. `vx` and `vy` are the horizontal
    /// and vertical velocity. `timeout` is the duration of the movement.
    pub fn continuous_move(&self, vx: f32, vy: f32, timeout: Duration) -> Result<()> {
        trace!(
            "continuous_move vx={}, vy={} timeout={:?}",
            &vx,
            &vy,
            &timeout
        );

        let soap_msg = onvif::soap_envelop(
            onvif::soap_body(onvif::continuous_move(&self.profile_token()?, vx, vy)),
            self.get_auth(),
        );

        let _ = self.post(&soap_msg)?;

        sleep(timeout); // todo use the timeout field in SOAP

        self.stop(true, true)?;
        Ok(())
    }

    /// Order a continuous move to the camera. `vx` and `vy` are the horizontal
    /// and vertical velocity. `timeout` is the duration of the movement.
    // Note: cannot test, my camera doesn't seem to support zoom (tried with onvif-device-tool)
    pub fn continuous_move_zoom(&self, vz: f32, timeout: Duration) -> Result<()> {
        trace!("continuous_move_zoom vz={} timeout={:?}", &vz, &timeout);

        let soap_msg = onvif::soap_envelop(
            onvif::soap_body(onvif::continuous_move_zoom(&self.profile_token()?, vz)),
            self.get_auth(),
        );

        let _ = self.post(&soap_msg)?;

        sleep(timeout); // todo use the timeout field in SOAP

        self.stop(true, true)?;
        Ok(())
    }

    /// Stop the pan-tilt and/or zoom camera movement.
    pub fn stop(&self, pantilt: bool, zoom: bool) -> Result<()> {
        trace!("stop pantilt={:?}, zoom={:?}", pantilt, zoom);

        let soap_msg = onvif::soap_envelop(
            onvif::soap_body(onvif::stop(self.profile_token()?, pantilt, zoom)),
            self.get_auth(),
        );

        let _ = self.post(&soap_msg)?;
        Ok(())
    }
}
