use crate::errors::{RetrievalError, FileNotFound};
use std::collections::HashMap;
use serde::Deserialize;

#[cfg(target_os="windows")]
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

#[cfg(target_os="windows")]
use wmi::{COMLibrary, WMIConnection, Variant};

#[cfg(target_os="windows")]
pub fn get_hwid() -> Result<String, RetrievalError>{
    let rkey = RegKey::predef(HKEY_LOCAL_MACHINE)
    .open_subkey("SOFTWARE\\Microsoft\\Cryptography")
    .expect("Failed to retrieve the hwid (Possible permission error)");
    let id = rkey.get_value("MachineGuid")
    .expect("Failed to retrieve the hwid (OS Error)");
    Ok(id)
}

#[derive(Deserialize, Debug)]
struct PhysicalMedia{
    SerialNumber: String,
}

#[cfg(target_os="windows")]
pub fn get_disk_id() -> Result<String, RetrievalError>{
    let con = WMIConnection::new(COMLibrary::new().unwrap().into())
    .expect("Could not establish WMI Connection");
    let ser:Vec<PhysicalMedia> = con
    .raw_query("SELECT SerialNumber FROM Win32_PhysicalMedia").expect("Error retrieving disk serial number");
    let serial = ser.get(0).unwrap().SerialNumber.clone();
    Ok(serial)
}