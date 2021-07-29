use std::{io::Read};
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use std::fs::File;
use std::fmt;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug)]
pub struct HwidRetrievalError;

#[derive(Debug)]
struct FileNotFound;

impl fmt::Display for HwidRetrievalError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Could not retrieve the HWID")
    }
}

impl HwidRetrievalError{
    fn new() -> Self{
        Self
    }
}

impl fmt::Display for FileNotFound{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Could not find the file")
    }
}

impl FileNotFound{
    fn new() -> Self{
        Self
    }
}

#[cfg(target_os="windows")]
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

#[cfg(target_os="windows")]
fn get_hwid() -> Result<String, HwidRetrievalError>{
    let rkey = RegKey::predef(HKEY_LOCAL_MACHINE)
    .open_subkey("SOFTWARE\\Microsoft\\Cryptography")
    .expect("Failed to retrieve the hwid (Possible permission error)");
    let id = rkey.get_value("MachineGuid")
    .expect("Failed to retrieve the hwid (OS Error)");
    Ok(id)
}

#[cfg(target_os="linux")]
const MACHINE_ID_FILES:[& str;2] = [
    "/var/lib/dbus/machine-id",
    "/etc/machine-id"
];

#[cfg(target_os="linux")]
fn get_file_content(path:&str) -> Result<String, FileNotFound>{
    let file_result = File::open(path);
    return match file_result{
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).expect("Could not read file contents");
            Ok(content)
        },
        Err(_) => Err(FileNotFound::new())
    }
}

#[cfg(target_os="linux")]
fn get_hwid() -> Result<String, HwidRetrievalError>{
    for path in MACHINE_ID_FILES.iter(){
        if std::path::Path::new(path).exists(){
            let content = get_file_content(path)
            .expect("Could not read file contents");
            return Ok(content)
        }
    }
    Err(HwidRetrievalError::new())
}

pub fn encrypted_id(key:&str) -> Result<String, HwidRetrievalError>{
    let id = get_hwid()
    .expect("Could not retrieve HWID");
    let mut mac = HmacSha256::new_from_slice(
        id.as_bytes()
    ).expect("Could not retrieve HWID");
    mac.update(key.as_bytes());
    let result = mac.finalize().into_bytes();
    let hex_result = format!("{:x}", result);
    Ok(hex_result)
}
