// app-machine-id - Generate app-specific machine IDs
// Copyright (C) 2023 d-k-bo
// SPDX-License-Identifier: LGPL-2.1-or-later

//! Generate app-specific machine IDs derived from the machine ID defined in
//! `/etc/machine-id` and an application ID.
//!
//! Unlike the default machine ID, which should be considered "confidential",
//! this implementation uses HMAC-SHA256 to generate an app-specific machine ID
//! which could be used in less secure contexts.
//!
//! This implementation is based on systemd's `sd_id128_get_machine_app_specific()`.
//!
//! See [`man machine-id(5)`](https://www.freedesktop.org/software/systemd/man/machine-id.html)
//! and [`man sd_id128_get_machine(3)`](https://www.freedesktop.org/software/systemd/man/sd_id128_get_machine_app_specific.html)
//! for details.

use std::{
    fmt::{Debug, Display},
    fs::read_to_string,
    io,
};

use hmac_sha256::HMAC;
use uuid::Uuid;

const MACHINE_ID_PATH: &str = "/etc/machine-id";

/// Generate an app-specific machine ID derived from the machine ID defined in
/// `/etc/machine-id` and an application ID.
///
/// Unlike the default machine ID, which should be considered "confidential",
/// this implementation uses HMAC-SHA256 to generate an app-specific machine UUID
/// which could be used in less secure contexts.
///
/// This implementation is based on systemd's `sd_id128_get_machine_app_specific()`.
///
/// See [`man machine-id(5)`](https://www.freedesktop.org/software/systemd/man/machine-id.html)
/// and [`man sd_id128_get_machine(3)`](https://www.freedesktop.org/software/systemd/man/sd_id128_get_machine_app_specific.html)
/// for details.
pub fn get(app_id: Uuid) -> Result<Uuid, Error> {
    let machine_id = machine_id()?;
    let hmac = HMAC::mac(app_id, machine_id);
    let id = Uuid::from_slice(&hmac[0..16])?;
    let id = make_v4_uuid(id);
    Ok(id)
}

fn machine_id() -> Result<Uuid, Error> {
    let id = Uuid::try_parse(read_to_string(MACHINE_ID_PATH)?.trim_end())?;
    Ok(id)
}

/// Turn the ID into a valid UUIDv4.
///
/// This code is inspired by `generate_random_uuid()` of drivers/char/random.c from the Linux kernel sources.
fn make_v4_uuid(id: Uuid) -> Uuid {
    let mut id = id.into_bytes();

    // Set UUID version to 4 --- truly random generation
    id[6] = (id[6] & 0x0F) | 0x40;
    // Set the UUID variant to DCE
    id[8] = (id[8] & 0x3F) | 0x80;

    Uuid::from_bytes(id)
}

#[derive(Debug)]
/// Returned when reading the machine ID fails.
pub enum Error {
    /// Could not read `/etc/machine-id`.
    Io(io::Error),
    /// The machine ID doesn't match the machine-id(5) format.
    InvalidId(uuid::Error),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "Could not read {MACHINE_ID_PATH}: {err}"),
            Self::InvalidId(_) => {
                write!(
                    f,
                    "The machine ID in {MACHINE_ID_PATH} does not \
                        match the format descibed in machine-id(5)"
                )
            }
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            Self::Io(err) => err,
            Self::InvalidId(err) => err,
        })
    }
}
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}
impl From<uuid::Error> for Error {
    fn from(err: uuid::Error) -> Self {
        Self::InvalidId(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static APP_ID: Uuid = uuid::uuid!("8e9b38ad-0ef8-4b14-894a-83bef002c713");

    #[test]
    fn test_machine_id() {
        let systemd = Uuid::try_parse_ascii(
            &std::process::Command::new("systemd-id128")
                .args(["machine-id"])
                .output()
                .unwrap()
                .stdout[0..32],
        )
        .unwrap();
        let this = machine_id().unwrap();
        assert_eq!(systemd, this);
    }

    #[test]
    fn test_app_specific_machine_id() {
        let systemd = Uuid::try_parse_ascii(
            &std::process::Command::new("systemd-id128")
                .args(["machine-id", "--app-specific", &APP_ID.to_string()])
                .output()
                .unwrap()
                .stdout[0..32],
        )
        .unwrap();
        let this = get(APP_ID).unwrap();
        assert_eq!(systemd, this);
    }
}
