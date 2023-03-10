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
    std::thread::spawn(|| {
        let con = WMIConnection::new(COMLibrary::new().unwrap())?;
        let ser: Vec<DiskGeneric> =
            con.raw_query("SELECT SerialNumber FROM Win32_PhysicalMedia")?;
        let serial = ser
            .get(0)
            .ok_or(HWIDError::new("UuidError", "Could not retrieve Uuid"))?
            .serial_number
            .clone();
        Ok(serial)
    })
    .join()
    .unwrap()
}

#[cfg(target_os = "windows")]
pub(crate) fn get_mac_address() -> Result<String, HWIDError> {
    std::thread::spawn(|| {
        let con = WMIConnection::new(COMLibrary::new().unwrap())?;
        let mac: String = con
            .raw_query(
                "SELECT MACAddress FROM Win32_NetworkAdapterConfiguration WHERE IPEnabled = true",
            )
            .map_err(|_| HWIDError::new("MACAddress", "Could not retrieve Mac Address"))?
            .into_iter()
            .map(|x: MACGeneric| x.MACAddress)
            .collect();
        Ok(mac)
    })
    .join()
    .unwrap()
}
