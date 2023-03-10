#[cfg(target_os = "windows")]
use crate::errors::HWIDError;
#[cfg(target_os = "windows")]
use serde::Deserialize;

#[cfg(target_os = "windows")]
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

#[cfg(target_os = "windows")]
use wmi::{COMLibrary, WMIConnection};

#[cfg(target_os = "windows")]
pub fn get_hwid() -> Result<String, HWIDError> {
    let rkey =
        RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey("SOFTWARE\\Microsoft\\Cryptography")?;
    let id = rkey.get_value("MachineGuid")?;
    Ok(id)
}

#[cfg(target_os = "windows")]
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DiskGeneric {
    serial_number: String,
}

#[cfg(target_os = "windows")]
#[derive(Deserialize)]
struct MACGeneric {
    MACAddress: String,
}

#[cfg(target_os = "windows")]
pub(crate) fn get_disk_id() -> Result<String, HWIDError> {
    let con = WMIConnection::new(COMLibrary::new().unwrap().into())?;
    let ser: Vec<DiskGeneric> = con.raw_query("SELECT SerialNumber FROM Win32_PhysicalMedia")?;
    let serial = ser
        .get(0)
        .ok_or(HWIDError::new("UuidError", "Could not retrieve Uuid"))?
        .serial_number
        .clone();
    Ok(serial)
}

#[cfg(target_os = "windows")]
pub(crate) fn get_mac_address() -> Result<String, HWIDError> {
    let con = WMIConnection::new(COMLibrary::new().unwrap().into())?;
    let ser: Vec<MACGeneric> = con.raw_query("SELECT MACAddress from Win32_NetworkAdapter")?;
    Ok(ser
        .get(0)
        .ok_or(HWIDError::new(
            "MACAddress",
            "Could not retrieve Mac Address",
        ))?
        .MACAddress
        .clone())
}
