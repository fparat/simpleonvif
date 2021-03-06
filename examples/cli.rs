#[macro_use]
extern crate log;

use std::time::Duration;

use anyhow::{anyhow, Result};
use clap::Clap;

use simpleonvif::OnvifCamera;

#[derive(Clap, Debug)]
#[clap(setting = clap::AppSettings::AllowNegativeNumbers)]
struct ContinousMove {
    /// "Horizontal velocity"
    #[clap(allow_hyphen_values = true)]
    vx: f32,
    /// Vertical velocity
    #[clap(allow_hyphen_values = true)]
    vy: f32,
    /// Duration of movement in seconds
    #[clap(long, short = "t", default_value = "2")]
    timeout: f32, // seconds
}

#[derive(Clap, Debug)]
#[clap(setting = clap::AppSettings::AllowNegativeNumbers)]
struct ContinousZoom {
    /// Zoom speed (optional)
    #[clap(allow_hyphen_values = true)]
    vz: f32,
    /// Duration of movement in seconds
    #[clap(long, short = "t", default_value = "2")]
    timeout: f32, // seconds
}

#[derive(Clap, Debug)]
enum SubCommand {
    #[clap(name = "contmove", about = "Continous move")]
    ContinousMove(ContinousMove),
    #[clap(name = "contzoom", about = "Continous zoom")]
    ContinousZoom(ContinousZoom),
    #[clap(name = "getprofiles", about = "Get available profiles")]
    GetProfiles,
    #[clap(name = "stop", about = "Stop all camera movements")]
    Stop,
}

#[derive(Clap, Debug)]
#[clap(about = "simpleonvif CLI tool")]
struct Command {
    /// Camera address. Can include login information.
    /// Don't include "/onvif/device_service", it will be appended automatically.
    /// Example: http://username:password@10.87.0.45:3456
    address: String,
    #[clap(long, short = "p")]
    /// Profile token (required for some subcommands)
    profile: Option<String>,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

fn main() -> Result<()> {
    env_logger::init();

    let command = Command::parse();

    // Check if profile is given when mandatory
    if command.profile.is_none() {
        let is_mandatory = match &command.subcmd {
            SubCommand::ContinousMove(_) => true,
            SubCommand::ContinousZoom(_) => true,
            SubCommand::GetProfiles => false,
            SubCommand::Stop => true,
        };
        if is_mandatory {
            return Err(anyhow!(
                "missing profile token, provide it with --profile or -p"
            ));
        }
    }

    debug!("{:?}", &command);

    let address = format!("{}", &command.address);
    let profile = command.profile.as_ref().map(String::as_str);

    let cam = OnvifCamera::new(&address, profile)?;
    trace!("new camera: {:?}", &cam);

    match command.subcmd {
        SubCommand::ContinousMove(params) => {
            let timeout = Duration::from_secs_f32(params.timeout);
            info!(
                "moving vx={} vy={} for {:?}\n",
                params.vx, params.vy, timeout
            );
            cam.continuous_move(params.vx, params.vy, timeout)?;
        }
        SubCommand::ContinousZoom(params) => {
            let timeout = Duration::from_secs_f32(params.timeout);
            info!("zooming vz={} for {:?}\n", params.vz, timeout);
            cam.continuous_move_zoom(params.vz, timeout)?;
        }
        SubCommand::GetProfiles => {
            let profiles = cam.get_profiles()?;
            info!("found {} available profiles", profiles.len());
            for p in profiles {
                println!("{}", p);
            }
            println!();
        }
        SubCommand::Stop => cam.stop(true, true)?,
    }

    Ok(())
}
