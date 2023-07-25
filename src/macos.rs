#[cfg(target_os = "macos")]
use crate::errors::HWIDError;
#[cfg(target_os = "macos")]
use std::{process::Command, str};

#[cfg(target_os = "macos")]
pub(crate) fn get_hwid() -> Result<String, HWIDError> {
    let output = Command::new("ioreg")
        .arg("-d2")
        .arg("-c")
        .arg("IOPlatformExpertDevice")
        .output()?;

    let output_str = match str::from_utf8(&output.stdout) {
        Ok(it) => it,
        Err(err) => return Err(HWIDError::new(
            "UuidError",
            format!("Could not convert to utf8 string {}", err).as_str(),
        )),
    };

    let lines: Vec<&str> = output_str.lines().collect();
    for line in lines {
        if line.contains("IOPlatformUUID") {
            let parts: Vec<&str> = line.split("=").collect();
            if parts.len() != 2 {
                continue;
            }
            let uuid = parts[1]
                .trim()
                .trim_matches('"')
                .to_string();
            return Ok(uuid);
        }
    }

    Err(HWIDError::new(
        "UuidError",
        "Could not find IOPlatformUUID in the IORegistry",
    ))
}

#[cfg(target_os = "macos")]
pub(crate) fn get_mac_address() -> Result<String, HWIDError> {
    let output = Command::new("ifconfig")
        .arg("en0")
        .arg("ether")
        .output()?;

    let output_str = match str::from_utf8(&output.stdout) {
        Ok(it) => it,
        Err(err) => {
            return Err(HWIDError::new(
                    "UTF8Error",
                    format!("Could not convert to utf8 string {}", err).as_str(),
                ))
        },
    };

    let lines: Vec<&str> = output_str.lines().collect();
    for line in lines {
        if line.contains("ether") {
            let parts: Vec<&str> = line.split(" ").collect();
            if parts.len() >= 2 {
                let mac_address = parts[1].to_string();
                return Ok(mac_address);
            }
        }
    }

    Err(HWIDError::new(
        "MacAddressError",
        "Could not find MAC address",
    ))
}

#[cfg(target_os = "macos")]
pub(crate) fn get_disk_id() -> Result<String, HWIDError> {
    let output = Command::new("diskutil")
        .arg("info")
        .arg("/")
        .output()?;

    let output_str = match str::from_utf8(&output.stdout) {
        Ok(it) => it,
        Err(err) => return Err(HWIDError::new(
            "UuidError",
            format!("Could not convert to utf8 string {}", err).as_str(),
        )),
    };

    let lines: Vec<&str> = output_str.lines().collect();
    for line in lines {
        if line.contains("Volume UUID") {
            let parts: Vec<&str> = line.split(":").collect();
            if parts.len() != 2 {
                continue;
            }
            let uuid = parts[1]
                .trim()
                .to_string();
            return Ok(uuid);
        }
    }

    Err(HWIDError::new(
        "UuidError",
        "Could not find root disk's UUID",
    ))
}
