
use std::fs::OpenOptions;
use std::io::{Read, Write};
use uuid::Uuid;

pub(crate) fn get_file_contents(path:&str) -> String{
    let mut file = OpenOptions::new().write(true)
    .read(true).create(true).open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).expect("File contains invalid unreadable data");
    if content.is_empty(){
        let uuid = Uuid::new_v4().to_string();
        file.write_all(uuid.as_bytes()).unwrap();
        content = uuid.to_string();
    }
    return content
}