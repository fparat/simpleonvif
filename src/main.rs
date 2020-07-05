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
    /// Zoom speed (optional)
    #[clap(allow_hyphen_values = true)]
    vz: Option<f32>,
    /// Duration of movement in seconds
    #[clap(long, short = "t", default_value = "2")]
    timeout: f32, // seconds
}

#[derive(Clap, Debug)]
enum SubCommand {
    #[clap(name = "contmove", about = "Continous move")]
    ContinousMove(ContinousMove),
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
        };
        if is_mandatory {
            return Err(anyhow!(
                "missing profile token, provide it with --profile or -p"
            ));
        }
    }

    debug!("{:?}", &command);

    let address = format!("{}/onvif/device_service", &command.address);
    let profile = command.profile.as_ref().map(String::as_str);

    let cam = OnvifCamera::new(&address, profile);

    if let Ok(profiles) = cam.get_profiles() {
        info!("found {} available profiles", profiles.len());
        for p in profiles {
            println!("{}", p);
        }
        println!();
    }

    match command.subcmd {
        SubCommand::ContinousMove(params) => {
            let timeout = Duration::from_secs_f32(params.timeout);
            info!(
                "moving vx={} vy={} for {:?}\n",
                params.vx, params.vy, timeout
            );
            cam.continuous_move(params.vx, params.vy, timeout)?;
        }
    }

    Ok(())
}
