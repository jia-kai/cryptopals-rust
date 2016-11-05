//! This module provides a representation for binary block data and helpers to
//! manipulate binary data

extern crate rustc_serialize;

use super::error::CryptoError;

use self::rustc_serialize::base64::{self, ToBase64};
use self::rustc_serialize::hex::{FromHex, ToHex};

use std::ops::{BitXor, BitXorAssign};
use std::iter;
use std::slice;

pub struct Binary {
    data: Vec<u8>
}

impl Binary {
    /* ------------ constructors ------------ */
    pub fn new() -> Binary {
        Self::from_bytes(Vec::new())
    }

    pub fn from_bytes(data: Vec<u8>) -> Binary {
        Binary { data: data }
    }

    pub fn from_hex(hex: &str) -> Result<Binary, CryptoError> {
        match hex.from_hex() {
            Ok(data) => Ok(Self::from_bytes(data)),
            Err(err) => Err(CryptoError::from_msg(
                    format!("failed to convert from hex: {}", err).as_str()))
        }
    }

    /* ------------ accessors ------------ */
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn raw_data(&self) -> &Vec<u8> {
        &self.data
    }

    /* ------------ convert ------------ */
    pub fn to_base64(&self) -> String {
        self.data.to_base64(base64::STANDARD)
    }

    pub fn to_hex(&self) -> String {
        self.data.to_hex()
    }

    /// convert to string by interpreting the content as utf8
    pub fn to_str_utf8(&self) -> Result<String, CryptoError> {
        match String::from_utf8(self.data.clone()) {
            Ok(s) => Ok(s),
            Err(err) => Err(CryptoError::from_msg(
                    format!("failed to convert string: {}", err).as_str()))
        }
    }

    /* ------------ computations ------------ */
    /// get number of occurrences for each char
    ///
    /// # Return value
    /// a vector of exactly 256 elements
    pub fn histogram(&self) -> Vec<usize> {
        let mut ret = vec![0usize; 256];
        for i in &self.data {
            ret[*i as usize] += 1;
        }
        ret
    }
}

// a better impl could utilize BIter = iter::Map<I, F>, but I just can not get
// the correct type for F, since:
//  0. rust lacks auto type deduce
//  1. lambdas have anonymous types and can not be used here
//  2. impl Fn<(&u8), u8> for custom proxy struct, but rustc fails by saying
//     format of `Fn`-family traits is unstable and refers to #29625

/// maps Iterator<&u8> to Iterator<u8>, equivalent to iter.map(|x| *x), but
/// with an explicit type name
pub struct U8IterRefRm<T> {
    iter: T
}

impl<'a, T> Iterator for U8IterRefRm<T>
        where T: Iterator<Item=&'a u8> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(v) => Some(*v),
            None    => None
        }
    }
}

/// a binary stream
pub trait BinaryIter {
    type BIter: Iterator<Item=u8>;

    fn biter(self) -> Self::BIter;
}

impl<'a> BinaryIter for &'a Binary {
    type BIter = U8IterRefRm<slice::Iter<'a, u8>>;

    fn biter(self) -> Self::BIter {
         U8IterRefRm { iter: self.data.iter() }
    }
}

impl BinaryIter for u8 {
    type BIter = iter::Repeat<u8>;
    fn biter(self) -> Self::BIter {
        iter::repeat(self)
    }
}

pub struct BinaryIterMaker<T> where T: Iterator<Item=u8> {
    iter: T
}

impl<T> BinaryIterMaker<T> where T: Iterator<Item=u8> {
    pub fn new(iter: T) -> BinaryIterMaker<T> {
        BinaryIterMaker { iter: iter }
    }
}

impl<T> BinaryIter for BinaryIterMaker<T> where T: Iterator<Item=u8> {
    type BIter = T;
    fn biter(self) -> Self::BIter {
        self.iter
    }
}

/// xor with an iterator of u8
impl<'a, T> BitXor<T> for &'a Binary where T: BinaryIter {
    type Output = Binary;

    fn bitxor(self, rhs: T) -> Binary {
        let ret = Binary::from_bytes(
            self.data.iter()
            .zip(rhs.biter())
            .map(|(x, y)| x ^ y)
            .collect::<Vec<_>>());
        assert_eq!(ret.len(), self.data.len());
        ret
    }
}

/// xor_assign with an iterator of u8
impl<T> BitXorAssign<T> for Binary where T: BinaryIter {
    fn bitxor_assign(&mut self, rhs: T) {
        let len = self.data.len();
        let mut data = &mut self.data;
        let mut riter = rhs.biter();
        for i in 0..len {
            data[i] ^= riter.next().unwrap();
        }
    }
}

