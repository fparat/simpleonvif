//! Namespace definitions

// XML
pub const XML_SCHEMA: &str = "http://www.w3.org/2001/XMLSchema";
pub const XML_SCHEMA_INSTANCE: &str = "http://www.w3.org/2001/XMLSchema-instance";

// SOAP
pub const SOAP_ENV: &str = "http://www.w3.org/2003/05/soap-envelope";

// Web services security
pub const WSS_PWDIGEST: &str = "http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-username-token-profile-1.0#PasswordDigest";
pub const WSS_BASE64BIN: &str = "http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-soap-message-security-1.0#Base64Binary";
pub const WSS_SECUTIL: &str =
    "http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd";
pub const WSS_SECEXT: &str =
    "http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd";

// ONVIF base namespaces for schema, device, media & ptz
pub const OVF_SCHEMA: &str = "http://www.onvif.org/ver10/schema";
pub const OVF_DEVICE: &str = "http://www.onvif.org/ver10/device/wsdl";
pub const OVF_MEDIA: &str = "http://www.onvif.org/ver10/media/wsdl";
pub const OVF_PTZ: &str = "http://www.onvif.org/ver20/ptz/wsdl";

// ONVIF zoom
pub const OVF_ZS_VGS: &str = "http://www.onvif.org/ver10/tptz/ZoomSpaces/VelocityGenericSpace";
pub const OVF_ZS_TGS: &str = "http://www.onvif.org/ver10/tptz/ZoomSpaces/TranslationGenericSpace";
pub const OVF_ZS_PGS: &str = "http://www.onvif.org/ver10/tptz/ZoomSpaces/PositionGenericSpace";
pub const OVF_ZS_ZGSS: &str = "http://www.onvif.org/ver10/tptz/ZoomSpaces/ZoomGenericSpeedSpace";

// ONVIF pan & tilt
pub const OVF_PTS_VGS: &str = "http://www.onvif.org/ver10/tptz/PanTiltSpaces/VelocityGenericSpace";
pub const OVF_PTS_TGS: &str =
    "http://www.onvif.org/ver10/tptz/PanTiltSpaces/TranslationGenericSpace";
pub const OVF_PTS_PGS: &str = "http://www.onvif.org/ver10/tptz/PanTiltSpaces/PositionGenericSpace";
pub const OVF_PTS_GSS: &str = "http://www.onvif.org/ver10/tptz/PanTiltSpaces/GenericSpeedSpace";
