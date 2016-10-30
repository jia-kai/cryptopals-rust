//! error handling helpers

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CryptoError {
    msg: String
}

impl CryptoError {
    pub fn from_msg(msg: &str) -> CryptoError {
        CryptoError { msg: msg.to_string() }
    }
}

impl Error for CryptoError {
    fn description(&self) -> &str {
        self.msg.as_str()
    }
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CryptoError({})", self.msg)
    }
}

