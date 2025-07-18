use std::process::Command;
use crate::errors::HWIDError;
use sha2::{Sha256, Digest};
use hex;
use std::os::unix::fs::MetadataExt;

pub fn get_hwid() -> Result<String, HWIDError> {
    let device = Command::new("getprop")
        .arg("ro.product.device")
        .output()?
        .stdout;
    let device_str = String::from_utf8(device)?.trim().to_string();

    let hwid = Command::new("getprop")
        .arg("ro.boot.cdt_hwid")
        .output()?
        .stdout;
    let hwid_str = String::from_utf8(hwid)?.trim().to_string();

    let combined = format!("{}-{}", device_str, hwid_str);

    let mut hasher = Sha256::new();
    hasher.update(combined.as_bytes());
    let result = hasher.finalize();

    Ok(hex::encode(result)[..32].to_string())
}

pub fn get_mac_address() -> Result<String, HWIDError> {
    let output = Command::new("getprop")
        .arg("ro.boot.wifimacaddr")
        .output()?;
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

pub fn get_disk_id() -> Result<String, HWIDError> {
    let path = "/data/data/com.termux";
    let metadata = std::fs::metadata(path)
        .map_err(|e| HWIDError::new("InvalidContent", &format!("Failed to get metadata for {}: {}", path, e)))?;

    let inode = metadata.ino();

    let mut hasher = Sha256::new();
    hasher.update(inode.to_string().as_bytes());
    let result = hasher.finalize();

    Ok(hex::encode(result)[..32].to_string())
}