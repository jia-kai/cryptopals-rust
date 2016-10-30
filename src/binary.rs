//! This module provides a representation for binary block data and helpers to
//! manipulate binary data

extern crate rustc_serialize;

use super::error::CryptoError;

use self::rustc_serialize::base64::{self, ToBase64};
use self::rustc_serialize::hex::{FromHex, ToHex};

use std::ops::{BitXor, BitXorAssign};

pub struct Binary {
    data: Vec<u8>
}

impl Binary {
    pub fn new() -> Binary {
        Self::from_data(Vec::new())
    }

    pub fn from_data(data: Vec<u8>)  -> Binary {
        Binary { data: data }
    }

    pub fn from_hex(hex: &str) -> Result<Binary, CryptoError> {
        match hex.from_hex() {
            Ok(data) => Ok(Self::from_data(data)),
            Err(err) => Err(CryptoError::from_msg(
                    format!("failed to convert from hex: {}", err).as_str()))
        }
    }

    pub fn to_base64(&self) -> String {
        self.data.to_base64(base64::STANDARD)
    }

    pub fn to_hex(&self) -> String {
        self.data.to_hex()
    }
}

impl<'a, 'b> BitXor<&'b Binary> for &'a Binary {
    type Output = Binary;

    fn bitxor(self, rhs: &'b Binary) -> Binary {
        assert_eq!(self.data.len(), rhs.data.len());
        Binary::from_data(
            self.data.iter().zip(rhs.data.iter())
            .map(|(x, y)| x ^ y)
            .collect::<Vec<_>>())
    }
}

impl<'a, 'b> BitXorAssign<&'b Binary> for &'a mut Binary {
    fn bitxor_assign(&mut self, rhs: &'b Binary) {
        let len = self.data.len();
        assert_eq!(len, rhs.data.len());
        let mut data = &mut self.data;
        for i in 0..len {
            data[i] ^= rhs.data[i];
        }
    }
}
