# simpleonvif

![CI](https://github.com/fparat/simpleonvif/workflows/Rust/badge.svg)

This library provides a simple client for interacting with ONVIF IP cameras.

Inspired by [PyOnvif](https://github.com/Pegax/pyOnvif), the implementation
is simplistic, and focus and basic functions. Use it for prototyping ("make
it work"), but do not expect full compliance of the ONVIF specifications. The
requests are blocking, and not much care is given to security for the moment.

## Example

```rust
use std::time::Duration;
use simpleonvif::OnvifCamera;

// Create a camera handle
let cam = OnvifCamera::new("http://user:passwd@192.168.0.32:8080", Some("profile1"))?;

// Fetch available profiles
let profiles = cam.get_profiles()?;

// Continous move right for 3 seconds
cam.continuous_move(1.0, 0.0, Duration::from_secs(3))?;
```

## CLI tool

A CLI tool is included as example

```
# Get usage help
cargo run --example cli -- --help


# Continous move right for 5 seconds
# (replace the url and profile_1 as needed)
cargo run --example cli -- http://username:password@10.0.2.45:8080 -p profile_1 contmove 1 0 -t 5
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Disclaimer

This library is not affiliated or associated in any way with ONVIF.

All product and company names are trademarks or registered trademarks of
their respective holders. Use of them does not imply any affiliation with or
endorsement by them.
