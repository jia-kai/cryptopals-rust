//! This module provides a representation for binary block data and helpers to
//! manipulate binary data

extern crate rustc_serialize;

use super::error::CryptoError;
use self::rustc_serialize::base64::{self, ToBase64};
use self::rustc_serialize::hex::FromHex;

pub struct Binary {
    data: Vec<u8>
}

impl Binary {
    pub fn new() -> Binary {
        Binary { data: Vec::new() }
    }

    pub fn from_hex(hex: &str) -> Result<Binary, CryptoError> {
        match hex.from_hex() {
            Ok(data) => Ok(Binary { data: data }),
            Err(err) => Err(CryptoError::from_msg(
                    format!("failed to convert from hex: {}", err).as_str()))
        }
    }

    pub fn to_base64(&self) -> String {
        self.data.to_base64(base64::STANDARD)
    }
}

