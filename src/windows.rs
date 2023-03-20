#[cfg(target_os = "windows")]
use crate::errors::HWIDError;
#[cfg(target_os = "windows")]
use serde::Deserialize;

#[cfg(target_os = "windows")]
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

#[cfg(target_os = "windows")]
use wmi::{COMLibrary, WMIConnection};

thread_local! {
    #[cfg(target_os="windows")]
    static COM_LIB:COMLibrary = COMLibrary::without_security().unwrap();
}

#[cfg(target_os = "windows")]
#[cfg(target_arch = "x86_64")]
pub fn get_hwid() -> Result<String, HWIDError> {
    let rkey =
        RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey("SOFTWARE\\Microsoft\\Cryptography")?;
    let id = rkey.get_value("MachineGuid")?;
    Ok(id)
}

#[cfg(target_os = "windows")]
#[cfg(target_arch = "x86")]
pub fn get_hwid() -> Result<String, HWIDError> {
    use winreg::enums::{KEY_READ, KEY_WOW64_64KEY};

    let rkey = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags("SOFTWARE\\Microsoft\\Cryptography",
        KEY_READ|KEY_WOW64_64KEY)?;

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
    let con = WMIConnection::new(COM_LIB.with(|con| *con))?;
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
    let con = WMIConnection::new(COM_LIB.with(|con| *con))?;
    let ser: Vec<MACGeneric> =
        con.raw_query("SELECT MACAddress from Win32_NetworkAdapter WHERE MACAddress IS NOT NULL")?;
    Ok(ser
        .get(0)
        .ok_or(HWIDError::new(
            "MACAddress",
            "Could not retrieve Mac Address",
        ))?
        .MACAddress
        .clone())
}
