use mockito;

// Connection: close
// Test the GetProfiles request, using captures from TP-LINK TL-IPC43AN-4
#[test]
fn test_get_profiles_tplink() {
    let resp_xml = include_str!("captures/get_profiles_response_tplink.xml");
    let url = mockito::server_url();
    let _mock = mockito::mock("POST", "/onvif/device_service")
        .with_header("Server", "gSOAP/2.8")
        .with_header("Content-Type", "application/soap+xml; charset=utf-8")
        .with_header("Connection", "close")
        .with_body(resp_xml)
        .create();

    let camera = simpleonvif::OnvifCamera::new(&url, None).unwrap();
    let profiles = camera.get_profiles().unwrap();
    assert_eq!(&profiles[..], &["profile_1", "profile_2"])
}

// Test the GetProfiles request, using captures from HZSOAR SOAR-970-230
#[test]
fn test_get_profiles_hzsoar() {
    let resp_xml = include_str!("captures/get_profiles_response_hzsoar.xml");
    let url = mockito::server_url();
    let _mock = mockito::mock("POST", "/onvif/device_service")
        .with_header("Content-Type", "application/soap+xml; charset=utf-8")
        .with_header("Connection", "close")
        .with_body(resp_xml)
        .create();

    let camera = simpleonvif::OnvifCamera::new(&url, None).unwrap();
    let profiles = camera.get_profiles().unwrap();
    assert_eq!(&profiles[..], &["MediaProfile000", "MediaProfile001"])
}
