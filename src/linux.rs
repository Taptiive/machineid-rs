#[cfg(target_os = "linux")]
use crate::errors::HWIDError;
#[cfg(target_os = "linux")]
use serde::Deserialize;
#[cfg(target_os = "linux")]
use std::fs::File;
#[cfg(target_os = "linux")]
use std::{io::Read, process::Command};

#[cfg(target_os = "linux")]
const MACHINE_ID_FILES: [&str; 2] = ["/var/lib/dbus/machine-id", "/etc/machine-id"];

#[cfg(target_os = "linux")]
#[derive(Deserialize)]
struct Output {
    blockdevices: Vec<Device>,
}

#[cfg(target_os = "linux")]
#[derive(Deserialize)]
struct Device {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    mountpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<Self>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uuid: Option<String>,
}

#[cfg(target_os = "linux")]
impl Output {
    #[cfg(target_os = "linux")]
    fn get_root(self) -> Result<String, HWIDError> {
        for devc in self.blockdevices.into_iter() {
            if let Some(mountpoint) = devc.mountpoint {
                if mountpoint.eq("/") {
                    // If the main disk is a sdcard, it's much safer to use the hardware cid over partition uuid
                    if devc.name.contains("mmc") {
                        let disk_name = &devc.name[0..devc.name.len() - 2];
                        let uuid =
                            std::fs::read_to_string(format!("/sys/block/{disk_name}/device/cid"))
                                .unwrap_or_default()
                                .trim()
                                .to_string();
                        if uuid.len() == 32 {
                            return Ok(uuid);
                        }
                    }
                    if let Some(uuid) = devc.uuid {
                        return Ok(uuid);
                    }
                }
            }
            if let Some(children) = devc.children {
                for chld_device in children.into_iter() {
                    if let Some(mnt) = chld_device.mountpoint {
                        if mnt.eq("/") {
                            if let Some(uuid) = chld_device.uuid {
                                return Ok(uuid);
                            }
                        }
                    }
                }
            }
        }
        Err(HWIDError::new(
            "UuidError",
            "Could not find root disk's UUID",
        ))
    }
}

#[cfg(target_os = "linux")]
pub(crate) fn get_disk_id() -> Result<String, HWIDError> {
    let output = run_command("lsblk -f -J -o NAME,MOUNTPOINT,UUID")?;

    let output_string = String::from_utf8(output.into())?;
    let parsed: Output = serde_json::from_str(output_string.as_str())?;
    let uuid = parsed.get_root()?;
    Ok(uuid)
}

#[cfg(target_os = "linux")]
fn run_command(command: &str) -> Result<String, HWIDError> {
    let mut cmd = Command::new("sh");
    let cmd = cmd.arg("-c").arg(command);

    let output = cmd.output()?;
    if !cmd.status()?.success() {
        return Err(HWIDError::new(
            &format!("Failed to run command: {command}"),
            &String::from_utf8(output.stderr.into())?,
        ));
    }

    Ok(String::from_utf8(cmd.output()?.stdout)?)
}

#[cfg(target_os = "linux")]
fn get_mac_addressof_interface(interface_name: &str) -> Result<String, HWIDError> {
    get_file_content(&format!("/sys/class/net/{interface_name}/address"))
}

#[cfg(target_os = "linux")]
pub(crate) fn get_mac_address() -> Result<String, HWIDError> {
    // First check for first cable internet interface connected on hardware,
    // if the ethernet cable is not available we find the first wifi interface.
    // From: https://www.freedesktop.org/wiki/Software/systemd/PredictableNetworkInterfaceNames/
    // Names incorporating Firmware/BIOS provided index numbers for on-board devices (example: eno1)
    // Names incorporating Firmware/BIOS provided PCI Express hotplug slot index numbers (example: ens1)
    // Names incorporating physical/geographical location of the connector of the hardware (example: enp2s0)
    // Names incorporating the interfaces's MAC address (example: enx78e7d1ea46da), we are going to ignore it for now
    // Classic, unpredictable kernel-native ethX naming (example: eth0)
    let first_cable_interface_names = [
        vec!["eno0".to_string(), "ens0".to_string()],
        (0..10)
            .map(|x| format!("enp{x}s0"))
            .collect::<Vec<String>>(),
        (0..10)
            .map(|x| format!("wlp{x}s0"))
            .collect::<Vec<String>>(),
        vec!["eth0".to_string()],
        vec!["wlan0".to_string()],
    ]
    .concat();

    for interface_name in first_cable_interface_names {
        let result = get_mac_addressof_interface(&interface_name);
        if result.is_ok() {
            return result;
        }
    }

    // If everything just fails, we get the (first) default network interface
    get_mac_addressof_interface(&run_command(
        "ip route show default | awk '/default/ {print $5; exit}'",
    )?)
}

#[cfg(target_os = "linux")]
fn get_file_content(path: &str) -> Result<String, HWIDError> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

#[cfg(target_os = "linux")]
pub(crate) fn get_hwid() -> Result<String, HWIDError> {
    for path in MACHINE_ID_FILES.iter() {
        if std::path::Path::new(path).exists() {
            let content = get_file_content(path)?.trim().to_string();
            return Ok(content);
        }
    }
    Err(HWIDError::new(
        "FileNotFound",
        "Could not find the files containing the System ID",
    ))
}
