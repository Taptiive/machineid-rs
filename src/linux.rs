use std::fs::File;
use std::{io::Read};
use crate::errors::{FileNotFound, RetrievalError};

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
pub fn get_hwid() -> Result<String, HwidRetrievalError>{
    for path in MACHINE_ID_FILES.iter(){
        if std::path::Path::new(path).exists(){
            let content = get_file_content(path)
            .expect("Could not read file contents");
            return Ok(content)
        }
    }
    Err(HwidRetrievalError::new())
}