use std::{error::Error, str::FromStr, string::FromUtf8Error};
use core::fmt::Display;

use crate::metadata::YKey;

#[derive(Default, Debug)]
pub struct YetiIOError {
    msg: String,
    key: YKey,
}

impl YetiIOError {
    pub fn set_key(&mut self, key: YKey) {
        self.key = key
    }
}

impl Clone for YetiIOError {
    fn clone(&self) -> Self {
        Self {
            msg: self.msg.clone(),
            key: self.key
        }
    }
}

impl From<String> for YetiIOError {
    fn from(msg: String) -> Self {
        Self {
            msg,
            ..Default::default()
        }
    }
}

impl From<&str> for YetiIOError {
    fn from(value: &str) -> Self {
        Self {
            msg: String::from_str(value).unwrap(),
            ..Default::default()
        }
    }
}

impl From<std::io::Error> for YetiIOError {
    fn from(e: std::io::Error) -> Self {
        Self {
            msg: format!("{}", e),
            ..Default::default()
        }
    }
}

impl From<FromUtf8Error> for YetiIOError {
    fn from(err: FromUtf8Error) -> Self {
        Self {
            msg: err.to_string(),
            ..Default::default()
        }
    }
}

impl From<xml::reader::Error> for YetiIOError {
    fn from(value: xml::reader::Error) -> Self {
        Self {
            msg: value.to_string(),
            ..Default::default()
        }
    }
}

impl Display for YetiIOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for YetiIOError { 
    fn description(&self) -> &str {
        &self.msg
    }
}