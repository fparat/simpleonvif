#[macro_use]
extern crate log;

use std::net::IpAddr;
use std::time::Duration;

use anyhow::Result;

use simpleonvif::OnvifCamera;

fn main() -> Result<()> {
    env_logger::init();

    let args = parse_args();
    let profile = args
        .value_of("profile")
        .map(String::from)
        .expect("missing profile");
    let ip: IpAddr = args
        .value_of("ip")
        .expect("missing ip")
        .parse()
        .expect("invalid ip");
    let port: u16 = args
        .value_of("port")
        .expect("missing port")
        .parse()
        .expect("invalid port");
    let user: Option<String> = args.value_of("user").map(String::from);
    let password: Option<String> = args.value_of("password").map(String::from);
    let vx = args
        .value_of("vx")
        .expect("missing vx")
        .parse::<f32>()
        .expect("not a float");
    let vy = args
        .value_of("vy")
        .expect("missing vy")
        .parse::<f32>()
        .expect("not a float");
    let timeout = args
        .value_of("seconds")
        .expect("missing time")
        .parse::<f32>()
        .map(|f| (f * 1000.) as u64) // to millis
        .map(Duration::from_millis) // to duration
        .unwrap();

    let address = format!("http://{}:{}/onvif/device_service", ip, port);

    let cam = OnvifCamera::new(&address, &profile)
        .with_user(user.as_ref())
        .with_password(password.as_ref());

    if let Ok(profiles) = cam.get_profiles() {
        info!("found {} available profiles", profiles.len());
        for p in profiles {
            println!("{}", p);
        }
        println!();
    }

    info!("moving vx={} vy={} for {:?}\n", vx, vy, timeout);
    cam.continuous_move(vx, vy, timeout)?;

    Ok(())
}

fn parse_args() -> clap::ArgMatches<'static> {
    clap::App::new(clap::crate_name!())
        .setting(clap::AppSettings::AllowLeadingHyphen) // allow negative values
        .max_term_width(80)
        .arg(
            clap::Arg::with_name("profile")
                .long("profile")
                .value_name("PROFILE")
                .help("profile token")
                .required(true)
        )
        .arg(
            clap::Arg::with_name("ip")
                .long("ip")
                .value_name("IP")
                .help("address ip")
                .required(true)
        )
        .arg(
            clap::Arg::with_name("port")
                .long("port")
                .value_name("PORT")
                .help("camera port")
                .required(true)
        )
        .arg(
            clap::Arg::with_name("user")
                .long("user")
                .value_name("user")
                .help("user name")
                .required(false),
        )
        .arg(
            clap::Arg::with_name("password")
                .long("password")
                .env("CAMERA_PASSWORD")
                .help("password")
                .required(false),
        )
        .arg(
            clap::Arg::with_name("vx")
                .value_name("VX")
                .help("vertical velocity")
                .required(true),
        )
        .arg(
            clap::Arg::with_name("vy")
                .value_name("VY")
                .help("vertical velocity")
                .required(true),
        )
        .arg(
            clap::Arg::with_name("seconds")
                .value_name("SECS")
                .default_value("2")
                .help("duration of command")
                .required(false),
        )
        .get_matches()
}
