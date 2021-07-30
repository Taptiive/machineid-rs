mod errors;
mod windows;
mod linux;

use errors::RetrievalError;

use itertools::Itertools;
#[cfg(target_os="linux")]
use linux::get_hwid;
#[cfg(target_os="windows")]
use windows::{get_hwid, get_disk_id};

use sysinfo::{SystemExt, System, ProcessorExt};
use crypto::{hmac::Hmac, mac::Mac, md5::Md5, sha1::Sha1, sha2::Sha256};
use std::collections::HashSet;
use indexmap::IndexMap;


pub enum Encryption{
    MD5,
    SHA256,
    SHA1
}

impl Encryption{
    fn generate_hash(&self, key:&[u8], text:String) -> String{
        match self{
            Encryption::MD5 => {
                let mut mac = Hmac::new(Md5::new(), key);
                mac.input(text.as_bytes());
                let hash = mac.result();
                hex::encode(hash.code())
            },
            Encryption::SHA1 => {
                let mut mac = Hmac::new(Sha1::new(), key);
                mac.input(text.as_bytes());
                let hash = mac.result();
                hex::encode(hash.code())
            },
            Encryption::SHA256 => {
                let mut mac = Hmac::new(Sha256::new(), key);
                mac.input(text.as_bytes());
                let hash = mac.result();
                hex::encode(hash.code())
            }
        }
    }
}
pub struct IdBuilder{
    parts: IndexMap<&'static str, String>,
    pub hash: Encryption,
}

impl IdBuilder{
    pub fn build(&mut self, key:&str) -> String{
        if self.parts.len() == 0 {
            panic!("You must add at least one element to make a machine id");
        }
        let set: HashSet<_> = self.parts.drain(..).collect();
        self.parts.extend(set.into_iter());
        let mut final_string = String::new();
        self.parts.iter().sorted().for_each(|(_,p)|final_string.push_str(p.as_str()));
        self.hash.generate_hash(key.as_bytes(), final_string)
    }
    #[cfg(target_os="windows")]
    pub fn add_drive_serial(&mut self) -> &mut IdBuilder{
        let serial = get_disk_id().unwrap();
        self.parts.insert("Drive Serial",serial);
        return self
    }
    pub fn add_cpu_id(&mut self) -> &mut IdBuilder{
        let sys = System::new_all();
        let processor = sys.global_processor_info();
        let id = processor.vendor_id();
        self.parts.insert("CPU Id",id.to_string());
        return self
    }
    pub fn add_machine_name(&mut self) -> &mut IdBuilder{
        let sys = System::new_all();
        let name = sys.host_name().expect("Unexpected error retrieving machine name");
        self.parts.insert("Machine Name",name);
        return self
    }
    pub fn add_username(&mut self) -> &mut IdBuilder{
        let username = whoami::username();
        self.parts.insert("Username", username);
        return self
    }
    pub fn add_os_name(&mut self) -> &mut IdBuilder{
        let sys = System::new_all();
        let name = sys.long_os_version().expect("Unexpected error retrieving OS name");
        self.parts.insert("OS Name",name);
        return self
    }
    pub fn add_cpu_cores(&mut self) -> &mut IdBuilder{
        let sys = System::new_all();
        let cores = sys.physical_core_count().unwrap_or(2);
        self.parts.insert("CPU Cores",cores.to_string());
        return self
    }
    pub fn add_system_id(&mut self) -> &mut IdBuilder{
        let id = get_hwid().expect("Unexpected error retrieving system id");
        self.parts.insert("System ID",id);
        return self
    }
    pub fn new(hash:Encryption) -> IdBuilder{
        IdBuilder{
            parts: IndexMap::new(),
            hash,
        }
    }
    
}
