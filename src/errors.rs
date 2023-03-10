use hmac::digest::InvalidLength;
#[cfg(target_os = "linux")]
use serde_json::Error as JsonError;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Error as IOError;
use std::string::FromUtf8Error;
#[cfg(target_os = "windows")]
use wmi::utils::WMIError;

#[derive(Debug)]
pub struct HWIDError {
    message: String,
    kind: String,
}

#[cfg(target_os = "windows")]
impl From<WMIError> for HWIDError {
    fn from(e: WMIError) -> Self {
        HWIDError {
            kind: String::from("WMIError"),
            message: format!("{}", e),
        }
    }
}

impl From<InvalidLength> for HWIDError {
    fn from(e: InvalidLength) -> Self {
        HWIDError {
            message: String::from("InvalidLength"),
            kind: e.to_string(),
        }
    }
}

#[cfg(target_os = "linux")]
impl From<JsonError> for HWIDError {
    fn from(e: JsonError) -> Self {
        HWIDError {
            kind: String::from("JsonError"),
            message: e.to_string(),
        }
    }
}

impl From<FromUtf8Error> for HWIDError {
    fn from(e: FromUtf8Error) -> Self {
        HWIDError {
            kind: String::from("Utf8Error"),
            message: e.to_string(),
        }
    }
}

impl From<IOError> for HWIDError {
    fn from(e: IOError) -> Self {
        HWIDError {
            kind: format!("{:?}", e.kind()),
            message: e.to_string(),
        }
    }
}

impl Error for HWIDError {}

impl Display for HWIDError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl HWIDError {
    pub(crate) fn new(reason: &str, message: &str) -> Self {
        HWIDError {
            kind: String::from(reason),
            message: String::from(message),
        }
    }
}
