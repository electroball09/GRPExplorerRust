use std::error::Error;
use core::fmt::Display;

#[derive(Debug)]
pub struct LoadError {
    msg: String,
    key: u32,
}

impl LoadError {
    pub fn new(msg: String, key: u32) -> Self {
        Self {
            msg, key
        }
    }

    pub fn set_key(&mut self, key: u32) {
        self.key = key
    }
}

impl Clone for LoadError {
    fn clone(&self) -> Self {
        Self {
            msg: self.msg.clone(),
            key: self.key
        }
    }
}

impl From<String> for LoadError {
    fn from(msg: String) -> Self {
        Self {
            msg,
            key: 0
        }
    }
}

impl From<std::io::Error> for LoadError {
    fn from(e: std::io::Error) -> Self {
        Self {
            msg: format!("{}", e),
            key: 0
        }
    }
}

impl Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for LoadError { 
    fn description(&self) -> &str {
        &self.msg
    }
}