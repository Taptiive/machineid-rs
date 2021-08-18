use std::fmt;

#[derive(Debug)]
pub struct RetrievalError;

#[derive(Debug)]
pub struct FileNotFound;

impl fmt::Display for RetrievalError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Could not retrieve the HWID")
    }
}

impl fmt::Display for FileNotFound{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Could not find the file")
    }
}