use std::{error::Error, str::FromStr, string::FromUtf8Error};
use core::fmt::Display;

use crate::metadata::YKey;

#[derive(Default, Debug)]
pub struct LoadError {
    msg: String,
    key: YKey,
}

impl LoadError {
    pub fn new(msg: String, key: YKey) -> Self {
        Self {
            msg, key
        }
    }

    pub fn set_key(&mut self, key: YKey) {
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
            ..Default::default()
        }
    }
}

impl From<&str> for LoadError {
    fn from(value: &str) -> Self {
        Self {
            msg: String::from_str(value).unwrap(),
            ..Default::default()
        }
    }
}

impl From<std::io::Error> for LoadError {
    fn from(e: std::io::Error) -> Self {
        Self {
            msg: format!("{}", e),
            ..Default::default()
        }
    }
}

impl From<FromUtf8Error> for LoadError {
    fn from(err: FromUtf8Error) -> Self {
        Self {
            msg: err.to_string(),
            ..Default::default()
        }
    }
}

impl From<xml::reader::Error> for LoadError {
    fn from(value: xml::reader::Error) -> Self {
        Self {
            msg: value.to_string(),
            ..Default::default()
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