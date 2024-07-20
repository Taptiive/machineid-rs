//! Get an encrypted unique MachineID/HWID/UUID.
//!
//! This crate is inspired by .Net DeviceId
//!
//! You can add all the components you need without admin permissions.
//!
//! ```
//! use machineid_rs::{IdBuilder, Encryption, HWIDComponent};
//!
//! // There are 3 different encryption types: MD5, SHA1 and SHA256.
//! let mut builder = IdBuilder::new(Encryption::MD5);
//!
//! builder.add_component(HWIDComponent::SystemID).add_component(HWIDComponent::CPUCores);
//!
//! let hwid = builder.build("mykey").unwrap();

#![allow(non_snake_case)]

mod errors;
mod linux;
mod macos;
mod utils;
mod windows;

use errors::HWIDError;
#[cfg(target_os = "linux")]
use linux::{get_disk_id, get_hwid, get_mac_address};
#[cfg(target_os = "macos")]
use macos::{get_disk_id, get_hwid, get_mac_address};
#[cfg(target_os = "windows")]
use windows::{get_disk_id, get_hwid, get_mac_address};

use hmac::{Hmac, Mac};
use md5::Md5;
use sha1::Sha1;
use sha2::Sha256;
use sysinfo::{CpuExt, System, SystemExt};
use utils::file_token;

/// The components that can be used to build the HWID.

#[derive(PartialEq, Eq, Hash)]
pub enum HWIDComponent {
    /// System UUID
    SystemID,
    /// Number of CPU Cores
    CPUCores,
    /// Name of the OS
    OSName,
    /// Current Username
    Username,
    /// Host machine name
    MachineName,
    /// Mac Address
    MacAddress,
    /// CPU Vendor ID
    CPUID,
    /// The contents of a file
    FileToken(&'static str),
    /// UUID of the root disk
    DriveSerial,
}

impl HWIDComponent {
    pub fn to_string(&self) -> Result<String, HWIDError> {
        use HWIDComponent::*;
        return match self {
            SystemID => get_hwid(),
            CPUCores => {
                let sys = System::new_all();
                let cores = sys.physical_core_count().unwrap_or(2);
                Ok(cores.to_string())
            }
            OSName => {
                let sys = System::new_all();
                let name = sys
                    .long_os_version()
                    .ok_or(HWIDError::new("OSName", "Could not retrieve OS Name"))?;
                Ok(name)
            }
            Username => Ok(whoami::username()),
            MachineName => {
                let sys = System::new_all();
                let name = sys
                    .host_name()
                    .ok_or(HWIDError::new("HostName", "Could not retrieve Host Name"))?;
                Ok(name)
            }
            MacAddress => get_mac_address(),
            CPUID => {
                let sys = System::new_all();
                let processor = sys.global_cpu_info();
                Ok(processor.vendor_id().to_string())
            }
            FileToken(filename) => file_token(filename),
            DriveSerial => get_disk_id(),
        };
    }
}

/// The encryptions that can be used to build the HWID.
pub enum Encryption {
    MD5,
    SHA256,
    SHA1,
}

type HmacMd5 = Hmac<Md5>;
type HmacSha1 = Hmac<Sha1>;
type HmacSha256 = Hmac<Sha256>;

impl Encryption {
    fn generate_hash(&self, key: &[u8], text: String) -> Result<String, HWIDError> {
        match self {
            Encryption::MD5 => {
                let mut mac = HmacMd5::new_from_slice(key)?;
                mac.update(text.as_bytes());
                let result = mac.finalize();
                Ok(hex::encode(result.into_bytes().as_slice()))
            }
            Encryption::SHA1 => {
                let mut mac = HmacSha1::new_from_slice(key)?;
                mac.update(text.as_bytes());
                let result = mac.finalize();
                Ok(hex::encode(result.into_bytes().as_slice()))
            }
            Encryption::SHA256 => {
                let mut mac = HmacSha256::new_from_slice(key)?;
                mac.update(text.as_bytes());
                let result = mac.finalize();
                Ok(hex::encode(result.into_bytes().as_slice()))
            }
        }
    }
}

/// `IdBuilder` is the constructor for the HWID. It can be used with the 3 different options of the `Encryption` enum.
pub struct IdBuilder {
    parts: Vec<HWIDComponent>,
    pub hash: Encryption,
}

impl IdBuilder {
    /// Joins every part together and returns a `Result` that may be the hashed HWID or a `HWIDError`.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if there is an error while retrieving the component's strings.
    ///
    /// # Examples
    ///
    /// ```
    /// use machineid_rs::{IdBuilder, Encryption, HWIDComponent};
    ///
    /// let mut builder = IdBuilder::new(Encryption::MD5);
    ///
    /// builder.add_component(HWIDComponent::SystemID);
    ///
    ///
    /// // Will panic if there is an error when the components return his values.
    /// let key = builder.build("mykey").unwrap();
    /// ```
    pub fn build(&mut self, key: &str) -> Result<String, HWIDError> {
        if self.parts.len() == 0 {
            panic!("You must add at least one element to make a machine id");
        }
        let final_string = self
            .parts
            .iter()
            .map(|p| p.to_string())
            .collect::<Result<String, HWIDError>>()?;
        self.hash.generate_hash(key.as_bytes(), final_string)
    }

    /// Adds a component to the `IdBuilder` that will be hashed once you call the [`IdBuilder::build`] function.
    ///
    /// You can't add the same component twice.
    ///
    /// # Examples
    ///
    /// ```
    /// use machineid_rs::{IdBuilder, Encryption, HWIDComponent};
    ///
    /// let mut builder = IdBuilder::new(Encryption::MD5);
    ///
    /// builder.add_component(HWIDComponent::SystemID);
    /// ```
    pub fn add_component(&mut self, component: HWIDComponent) -> &mut Self {
        if !self.parts.contains(&component) {
            self.parts.push(component);
        }
        return self;
    }

    /// Adds all possible components to the `IdBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use machineid_rs::{IdBuilder, Encryption};
    ///
    /// let mut builder = IdBuilder::new(Encryption::MD5);
    ///
    /// builder.add_all();
    /// ```
    ///
    /// It's the same as doing:
    ///
    /// ```
    /// use machineid_rs::{IdBuilder, Encryption, HWIDComponent};
    ///
    /// let mut builder = IdBuilder::new(Encryption::MD5);
    ///
    /// builder
    ///     .add_component(HWIDComponent::SystemID)
    ///     .add_component(HWIDComponent::OSName)
    ///     .add_component(HWIDComponent::CPUCores)
    ///     .add_component(HWIDComponent::CPUID)
    ///     .add_component(HWIDComponent::DriveSerial)
    ///     .add_component(HWIDComponent::MacAddress)
    ///     .add_component(HWIDComponent::Username)
    ///     .add_component(HWIDComponent::MachineName);
    ///
    /// ```
    pub fn add_all(&mut self) -> &mut Self {
        self.add_component(HWIDComponent::SystemID)
            .add_component(HWIDComponent::OSName)
            .add_component(HWIDComponent::CPUCores)
            .add_component(HWIDComponent::CPUID)
            .add_component(HWIDComponent::DriveSerial)
            .add_component(HWIDComponent::MacAddress)
            .add_component(HWIDComponent::Username)
            .add_component(HWIDComponent::MachineName)
    }

    /// Makes a new IdBuilder with the selected Encryption
    ///
    /// # Examples
    ///
    /// ```
    /// use machineid_rs::{IdBuilder, Encryption};
    ///
    /// let mut builder = IdBuilder::new(Encryption::MD5);
    /// ```
    pub fn new(hash: Encryption) -> Self {
        IdBuilder {
            parts: vec![],
            hash,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;
    #[test]
    fn every_option_sha256() {
        let mut builder = IdBuilder::new(Encryption::SHA256);
        builder
            .add_component(HWIDComponent::SystemID)
            .add_component(HWIDComponent::OSName)
            .add_component(HWIDComponent::CPUCores)
            .add_component(HWIDComponent::CPUID)
            .add_component(HWIDComponent::DriveSerial)
            .add_component(HWIDComponent::MacAddress)
            .add_component(HWIDComponent::FileToken("test.txt"))
            .add_component(HWIDComponent::Username)
            .add_component(HWIDComponent::MachineName);
        let hash = builder.build("mykey").unwrap();
        let expected = env::var("SHA256_MACHINEID_HASH").unwrap();
        assert_eq!(expected, hash);
    }

    #[test]
    fn every_option_sha1() {
        let mut builder = IdBuilder::new(Encryption::SHA1);
        builder
            .add_component(HWIDComponent::SystemID)
            .add_component(HWIDComponent::OSName)
            .add_component(HWIDComponent::CPUCores)
            .add_component(HWIDComponent::CPUID)
            .add_component(HWIDComponent::DriveSerial)
            .add_component(HWIDComponent::MacAddress)
            .add_component(HWIDComponent::FileToken("test.txt"))
            .add_component(HWIDComponent::Username)
            .add_component(HWIDComponent::MachineName);
        let hash = builder.build("mykey").unwrap();
        let expected = env::var("SHA1_MACHINEID_HASH").unwrap();
        assert_eq!(expected, hash);
    }

    #[test]
    fn every_option_md5() {
        let mut builder = IdBuilder::new(Encryption::MD5);
        builder
            .add_component(HWIDComponent::SystemID)
            .add_component(HWIDComponent::OSName)
            .add_component(HWIDComponent::CPUCores)
            .add_component(HWIDComponent::CPUID)
            .add_component(HWIDComponent::DriveSerial)
            .add_component(HWIDComponent::MacAddress)
            .add_component(HWIDComponent::FileToken("test.txt"))
            .add_component(HWIDComponent::Username)
            .add_component(HWIDComponent::MachineName);
        let hash = builder.build("mykey").unwrap();
        let expected = env::var("MD5_MACHINEID_HASH").unwrap();
        assert_eq!(expected, hash);
    }
}
