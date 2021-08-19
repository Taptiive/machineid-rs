mod errors;
mod windows;
mod linux;
mod utils;

use itertools::Itertools;
#[cfg(target_os="linux")]
use linux::{get_hwid, get_mac_address, get_disk_id};
#[cfg(target_os="windows")]
use windows::{get_hwid, get_disk_id, get_mac_address};

use sysinfo::{SystemExt, System, ProcessorExt};
use crypto::{hmac::Hmac, mac::Mac, md5::Md5, sha1::Sha1, sha2::Sha256};
use std::collections::HashSet;
use indexmap::IndexMap;
use utils::get_file_contents;


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
    pub fn add_drive_serial(&mut self) -> &mut Self{
        let serial = get_disk_id().unwrap();
        self.parts.insert("Drive Serial",serial);
        return self
    }
    pub fn add_file_token(&mut self, file_path:&str) -> &mut Self{
        let conts = get_file_contents(file_path);
        self.parts.insert("File Token", conts);
        return self
    }
    pub fn add_cpu_id(&mut self) -> &mut Self{
        let sys = System::new_all();
        let processor = sys.global_processor_info();
        let id = processor.vendor_id();
        self.parts.insert("CPU Id",id.to_string());
        return self
    }
    pub fn add_mac_address(&mut self) -> &mut Self{
        let addr = get_mac_address();
        self.parts.insert("MAC Address", addr);
        return self
    }
    pub fn add_machine_name(&mut self) -> &mut Self{
        let sys = System::new_all();
        let name = sys.host_name().expect("Unexpected error retrieving machine name");
        self.parts.insert("Machine Name",name);
        return self
    }
    pub fn add_username(&mut self) -> &mut Self{
        let username = whoami::username();
        self.parts.insert("Username", username);
        return self
    }
    pub fn add_os_name(&mut self) -> &mut Self{
        let sys = System::new_all();
        let name = sys.long_os_version().expect("Unexpected error retrieving OS name");
        self.parts.insert("OS Name",name);
        return self
    }
    pub fn add_cpu_cores(&mut self) -> &mut Self{
        let sys = System::new_all();
        let cores = sys.physical_core_count().unwrap_or(2);
        self.parts.insert("CPU Cores",cores.to_string());
        return self
    }
    pub fn add_system_id(&mut self) -> &mut Self{
        let id = get_hwid().expect("Unexpected error retrieving system id");
        self.parts.insert("System ID",id);
        return self
    }
    pub fn new(hash:Encryption) -> Self{
        IdBuilder{
            parts: IndexMap::new(),
            hash,
        }
    }
}

#[cfg(test)]
mod test{
    use super::Encryption;
    use super::IdBuilder;
    #[test]
    fn mac_address(){
        let mut a = IdBuilder::new(Encryption::SHA256);
        a.add_mac_address();
        let b = a.build("a");
        println!("{}", b);
    }
    #[test]
    fn file_token(){
        let mut a = IdBuilder::new(Encryption::SHA256);
        a.add_file_token("/home/bspc/test_file.txt");
        let b = a.build("a");
        println!("{}", b);
    }
    #[test]
    fn serial(){
        let mut a = IdBuilder::new(Encryption::SHA256);
        a.add_drive_serial();
        let b = a.build("a");
        println!("{}", b);
    }
}
