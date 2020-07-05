use std::thread::sleep;
use std::time::Duration;

use regex::Regex;
use ureq::Response;

use crate::auth;
use crate::error::Result;
use crate::{onvif, Error};

#[derive(Debug)]
pub struct OnvifCamera {
    address: String,
    profile: Option<String>,
    user: Option<String>,
    password: Option<String>,
}

impl OnvifCamera {
    pub fn new(address: &str, profile: Option<&str>) -> Self {
        let u = url::Url::parse(address).expect("invalid address"); // todo return a result

        let user = match u.username() {
            "" => None,
            name => Some(name.to_string()),
        };
        let password = u.password().map(String::from);

        Self {
            address: address.to_string(),
            profile: profile.map(String::from),
            user,
            password,
        }
    }

    pub fn with_user<S: Into<String>>(mut self, user: Option<S>) -> Self {
        self.user = user.map(|u| u.into());
        self
    }

    pub fn with_password<S: Into<String>>(mut self, password: Option<S>) -> Self {
        self.password = password.map(|p| p.into());
        self
    }

    fn get_auth(&self) -> Option<String> {
        match (&self.user, &self.password) {
            (Some(ref u), Some(ref p)) => Some(auth::get_auth_header(u, p)),
            _ => None,
        }
    }

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

    fn get_profile(&self) -> Result<&str> {
        self.profile
            .as_ref()
            .map(|s| s.as_str())
            .ok_or(Error::MissingProfile)
    }

    pub fn continuous_move(&self, vx: f32, vy: f32, timeout: Duration) -> Result<()> {
        trace!(
            "continuous_move vx={}, vy={} timeout={:?}",
            &vx,
            &vy,
            &timeout
        );

        let soap_msg = onvif::soap_envelop(
            onvif::soap_body(onvif::continuous_move(&self.get_profile()?, vx, vy)),
            self.get_auth(),
        );

        let _ = self.post(&soap_msg)?;

        sleep(timeout); // todo use the timeout field in SOAP

        self.stop(true, true)?;
        Ok(())
    }

    pub fn stop(&self, pantilt: bool, zoom: bool) -> Result<()> {
        trace!("stop pantilt={:?}, zoom={:?}", pantilt, zoom);

        let soap_msg = onvif::soap_envelop(
            onvif::soap_body(onvif::stop(self.get_profile()?, pantilt, zoom)),
            self.get_auth(),
        );

        let _ = self.post(&soap_msg)?;
        Ok(())
    }
}
