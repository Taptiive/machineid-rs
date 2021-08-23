
use std::fs::File;
use std::{io::Read, process::Command};
use crate::errors::{FileNotFound, RetrievalError};
use serde::Deserialize;

#[cfg(target_os="linux")]
const MACHINE_ID_FILES:[& str;2] = [
    "/var/lib/dbus/machine-id",
    "/etc/machine-id"
];

#[cfg(target_os="linux")]
#[derive(Deserialize)]
struct Output{
    blockdevices: Vec<Device>
}

#[cfg(target_os="linux")]
#[derive(Deserialize)]
struct Device{
    #[serde(skip_serializing_if = "Option::is_none")]
    mountpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<Self>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uuid: Option<String>
}

#[cfg(target_os="linux")]
impl Output{
    #[cfg(target_os="linux")]
    fn get_root(self) -> Result<String, RetrievalError>{
        for devc in self.blockdevices.into_iter(){
            if let Some(mountpoint) = devc.mountpoint{
                if mountpoint.eq("/") {
                    if let Some(uuid) = devc.uuid{
                        return Ok(uuid)
                    }
                }
            }
            if let Some(children) = devc.children{
                for chld_device in children.into_iter(){
                    if let Some(mnt) = chld_device.mountpoint{
                        if mnt.eq("/") {
                            if let Some(uuid) = chld_device.uuid{
                                return Ok(uuid)
                            }
                        }
                    }
                }
            }
        }
        Err(RetrievalError)
    }
}

#[cfg(target_os="linux")]
pub fn get_disk_id() -> Result<String, RetrievalError>{

    let mut com = Command::new("sh");
    com.arg("-c").arg("lsblk -f -J");

    let output = com.output().
    expect("Invalid command output");

    let output_string = String::from_utf8(output.stdout)
    .expect("Could not read invalid bytes");
    let parsed:Output = serde_json::from_str(output_string.as_str()).expect("Invalid JSON returned by command");
    let uuid = parsed.get_root().expect("Could not detect root disk");
    Ok(uuid)
}

#[cfg(target_os="linux")]
pub fn get_mac_address() -> String{
    let mut com = Command::new("sh");
    com.arg("-c").arg("cat /sys/class/net/$(ip route show default | awk '/default/ {print $5}')/address");
    let output = com.output().expect("Could not read output from command");
    String::from_utf8(output.stdout).expect("Invalid bytes in command output")
}

#[cfg(target_os="linux")]
fn get_file_content(path:&str) -> Result<String, FileNotFound>{
    let file_result = File::open(path);
    return match file_result{
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).expect("Could not read file contents");
            Ok(content)
        },
        Err(_) => Err(FileNotFound)
    }
}

#[cfg(target_os="linux")]
pub fn get_hwid() -> Result<String, RetrievalError>{
    for path in MACHINE_ID_FILES.iter(){
        if std::path::Path::new(path).exists(){
            let content = get_file_content(path)
            .expect("Could not read file contents");
            return Ok(content)
        }
    }
    Err(RetrievalError)
}