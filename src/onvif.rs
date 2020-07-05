//! Functions generating ONVIF SOAP messages

use std::fmt::Display;

use crate::namespaces::*;

pub fn soap_envelop(content: impl Display, auth: Option<impl Display>) -> String {
    if let Some(auth) = auth {
        format!(
            r#"
<s:Envelope xmlns:s="{SOAP_ENV}">{auth}{content}</s:Envelope>
"#,
            SOAP_ENV = SOAP_ENV,
            auth = auth,
            content = content
        )
    } else {
        format!(
            r#"
<s:Envelope xmlns:s="{SOAP_ENV}">{content}</s:Envelope>
"#,
            SOAP_ENV = SOAP_ENV,
            content = content
        )
    }
}

pub fn soap_body(content: impl Display) -> String {
    format!(
        r#"
<s:Body xmlns:xsi="{XML_SCHEMA_INSTANCE}" xmlns:xsd="{XML_SCHEMA}">{content}</s:Body>
"#,
        XML_SCHEMA_INSTANCE = XML_SCHEMA_INSTANCE,
        XML_SCHEMA = XML_SCHEMA,
        content = content
    )
}

pub fn auth_header(
    username: impl Display,
    pdigest: impl Display,
    nonce: impl Display,
    created: impl Display,
) -> String {
    format!(
        r#"
<s:Header>
    <Security s:mustUnderstand="1" xmlns="{WSS_SECEXT}">
        <UsernameToken>
            <Username>{username}</Username>
            <Password Type="{WSS_PWDIGEST}">{pdigest}</Password>
            <Nonce EncodingType="{WSS_BASE64BIN}">{nonce}</Nonce>
            <Created xmlns="{WSS_SECUTIL}">{created}</Created>
        </UsernameToken>
    </Security>
</s:Header>
"#,
        WSS_SECEXT = WSS_SECEXT,
        username = username,
        WSS_PWDIGEST = WSS_PWDIGEST,
        pdigest = pdigest,
        WSS_BASE64BIN = WSS_BASE64BIN,
        nonce = nonce,
        WSS_SECUTIL = WSS_SECUTIL,
        created = created,
    )
}

pub fn get_system_datetime() -> String {
    format!(
        r#"
<GetSystemDateAndTime xmlns="{OVF_DEVICE}"/>
"#,
        OVF_DEVICE = OVF_DEVICE
    )
}

pub fn get_capabilities() -> String {
    format!(
        r#"
<GetCapabilities xmlns="{OVF_DEVICE}"><Category>All</Category></GetCapabilities>
"#,
        OVF_DEVICE = OVF_DEVICE
    )
}

pub fn get_service_capabilities() -> String {
    format!(
        r#"
<GetServiceCapabilities xmlns="{OVF_DEVICE}"></GetServiceCapabilities>
"#,
        OVF_DEVICE = OVF_DEVICE
    )
}

pub fn get_services() -> String {
    format!(
        r#"
<GetServices xmlns="{OVF_DEVICE}"><IncludeCapability>false</IncludeCapability></GetServices>
"#,
        OVF_DEVICE = OVF_DEVICE
    )
}

pub fn get_profiles() -> String {
    format!(
        r#"
<GetProfiles xmlns="{OVF_MEDIA}"></GetProfiles>
"#,
        OVF_MEDIA = OVF_MEDIA
    )
}

pub fn get_device_info() -> String {
    format!(
        r#"
<GetDeviceInformation xmlns="{OVF_DEVICE}"></GetDeviceInformation>
"#,
        OVF_DEVICE = OVF_DEVICE
    )
}

pub fn get_node(node: impl Display) -> String {
    format!(
        r#"
<GetNode xmlns="{OVF_PTZ}">
    <NodeToken>{node}</NodeToken>
</GetNode>
"#,
        OVF_PTZ = OVF_PTZ,
        node = node
    )
}

pub fn relative_mode(profile: impl Display, x: f32, y: f32, xspeed: f32, yspeed: f32) -> String {
    format!(
        r#"
<RelativeMove xmlns="{OVF_PTZ}">
    <ProfileToken>{profile}</ProfileToken>
    <Translation>
        <PanTilt x="{x}" y="{y}" space="{OVF_PTS_TGS}" xmlns="{OVF_SCHEMA}"/>
    </Translation>
    <Speed>
        <PanTilt x="{xspeed}" y="{yspeed}" space="{OVF_PTS_GSS}" xmlns="{OVF_SCHEMA}"/>
    </Speed>
</RelativeMove>
"#,
        OVF_PTZ = OVF_PTZ,
        profile = profile,
        x = x,
        y = y,
        OVF_PTS_TGS = OVF_PTS_TGS,
        OVF_SCHEMA = OVF_SCHEMA,
        xspeed = xspeed,
        yspeed = yspeed,
        OVF_PTS_GSS = OVF_PTS_GSS,
    )
}

pub fn relative_move_zoom(profile: impl Display, z: f32, zspeed: f32) -> String {
    format!(
        r#"
<RelativeMove xmlns="{OVF_PTZ}">
    <ProfileToken>{profile}</ProfileToken>
    <Translation>
        <Zoom x="{z}" space="{OVF_ZS_TGS}" xmlns="{OVF_SCHEMA}"/>
    </Translation>
    <Speed>
        <Zoom x="{zspeed}" space="{OVF_ZS_ZGSS}" xmlns="{OVF_SCHEMA}"/>
    </Speed>
</RelativeMove>
"#,
        OVF_PTZ = OVF_PTZ,
        profile = profile,
        z = z,
        OVF_ZS_TGS = OVF_ZS_TGS,
        OVF_SCHEMA = OVF_SCHEMA,
        zspeed = zspeed,
        OVF_ZS_ZGSS = OVF_ZS_ZGSS,
    )
}

pub fn absolute_move(profile: impl Display, x: f32, y: f32, z: f32) -> String {
    format!(
        r#"
<AbsoluteMove xmlns="{OVF_PTZ}">
    <Position>
        <ProfileToken>{profile}</ProfileToken>
        <PanTilt x="{x}" y="{y}" space="{OVF_PTS_PGS}" xmlns="{OVF_SCHEMA}"/>
        <Zoom x="{z}" space="{OVF_ZS_PGS}" xmlns="{OVF_SCHEMA}"/>
    </Position>
</AbsoluteMove>
"#,
        OVF_PTZ = OVF_PTZ,
        profile = profile,
        x = x,
        y = y,
        OVF_PTS_PGS = OVF_PTS_PGS,
        OVF_SCHEMA = OVF_SCHEMA,
        z = z,
        OVF_ZS_PGS = OVF_ZS_PGS,
    )
}

pub fn continuous_move(profile: impl Display, x: f32, y: f32) -> String {
    format!(
        r#"
<ContinuousMove xmlns="{OVF_PTZ}">
    <ProfileToken>{profile}</ProfileToken>
    <Velocity>
        <PanTilt x="{x}" y="{y}" space="{OVF_PTS_VGS}" xmlns="{OVF_SCHEMA}"/>
    </Velocity>
</ContinuousMove>
"#,
        OVF_PTZ = OVF_PTZ,
        profile = profile,
        x = x,
        y = y,
        OVF_PTS_VGS = OVF_PTS_VGS,
        OVF_SCHEMA = OVF_SCHEMA
    )
}

pub fn stop(profile: impl Display, ptstop: bool, zstop: bool) -> String {
    format!(
        r#"
<Stop xmlns="{OVF_PTZ}">
    <ProfileToken>{profile}</ProfileToken>
    <PanTilt>{ptstop}</PanTilt>
    <Zoom>{zstop}</Zoom>
</Stop>
"#,
        OVF_PTZ = OVF_PTZ,
        profile = profile,
        ptstop = ptstop,
        zstop = zstop,
    )
}

pub fn continuous_move_zoom(profile: impl Display, z: f32) -> String {
    format!(
        r#"
<ContinuousMove xmlns="{OVF_PTZ}">
    <ProfileToken>{profile}</ProfileToken>
    <Velocity>
        <Zoom x="{z}" space="{OVF_ZS_VGS}" xmlns="{OVF_SCHEMA}"/>
    </Velocity>
</ContinuousMove>
"#,
        OVF_PTZ = OVF_PTZ,
        profile = profile,
        z = z,
        OVF_ZS_VGS = OVF_ZS_VGS,
        OVF_SCHEMA = OVF_SCHEMA
    )
}

pub fn set_preset(profile: impl Display, preset: u32) -> String {
    format!(
        r#"
    <SetPreset xmlns="{OVF_PTZ}">
        <ProfileToken>{profile}</ProfileToken>
        <PresetName>{preset}</PresetName>
    </SetPreset>
"#,
        OVF_PTZ = OVF_PTZ,
        profile = profile,
        preset = preset,
    )
}

pub fn get_presets(profile: impl Display) -> String {
    format!(
        r#"
<GetPresets xmlns="{OVF_PTZ}">
    <ProfileToken>{profile}</ProfileToken>
</GetPresets>
"#,
        OVF_PTZ = OVF_PTZ,
        profile = profile
    )
}

pub fn goto_preset(
    profile: impl Display,
    preset: u32,
    xspeed: f32,
    yspeed: f32,
    zspeed: f32,
) -> String {
    format!(
        r#"
<GotoPreset xmlns="{OVF_PTZ}">
    <ProfileToken>{profile}</ProfileToken>
    <PresetToken>{preset}</PresetToken>
    <Speed>
        <PanTilt x="{xspeed}" y="{yspeed}" xmlns="{OVF_SCHEMA}"/>
        <Zoom x="{zspeed}" xmlns="{OVF_SCHEMA}"/>
    </Speed>
</GotoPreset>
"#,
        OVF_PTZ = OVF_PTZ,
        profile = profile,
        preset = preset,
        xspeed = xspeed,
        yspeed = yspeed,
        OVF_SCHEMA = OVF_SCHEMA,
        zspeed = zspeed,
    )
}

pub fn remove_preset(profile: impl Display, preset: impl Display) -> String {
    format!(
        r#"
<RemovePreset xmlns="{OVF_PTZ}">
    <ProfileToken>{profile}</ProfileToken>
    <PresetToken>{preset}</PresetToken>
</RemovePreset>
"#,
        OVF_PTZ = OVF_PTZ,
        profile = profile,
        preset = preset
    )
}

pub fn get_video_sources() -> String {
    format!(
        r#"
<GetVideoSources xmlns="{OVF_MEDIA}"/>
"#,
        OVF_MEDIA = OVF_MEDIA
    )
}

pub fn get_stream_uri(profile: impl Display) -> String {
    format!(
        r#"
<GetStreamUri xmlns="{OVF_MEDIA}">
    <StreamSetup>
        <Stream xmlns="{OVF_SCHEMA}">RTP-Unicast</Stream>
        <Transport xmlns="{OVF_SCHEMA}">
            <Protocol>UDP</Protocol>
        </Transport>
    </StreamSetup>
    <ProfileToken>{profile}</ProfileToken>
</GetStreamUri>"#,
        OVF_MEDIA = OVF_MEDIA,
        OVF_SCHEMA = OVF_SCHEMA,
        profile = profile
    )
}
