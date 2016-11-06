//! This module provides representations and helpers for manipulating
//! contiguous binary data

extern crate rustc_serialize;

use super::error::CryptoError;

use self::rustc_serialize::base64::{self, FromBase64, ToBase64};
use self::rustc_serialize::hex::{FromHex, ToHex};

use std::ops::{BitXor, BitXorAssign};
use std::iter;
use std::slice;

/// algorithms on binary data
pub trait BinaryAlgo {

    /* ------------ abstract methods ------------ */
    fn as_slice(&self) -> &[u8];

    /* ------------ accessors ------------ */
    #[inline]
    fn len(&self) -> usize {
        self.as_slice().len()
    }

    /* ------------ convert ------------ */
    fn to_base64(&self) -> String {
        self.as_slice().to_base64(base64::STANDARD)
    }

    fn to_hex(&self) -> String {
        self.as_slice().to_hex()
    }

    /// convert to string by interpreting the content as utf8
    fn to_str_utf8(&self) -> Result<String, CryptoError> {
        match String::from_utf8(self.as_slice().to_vec()) {
            Ok(s) => Ok(s),
            Err(err) => Err(CryptoError::from_msg(
                    format!("failed to convert string: {}", err).as_str()))
        }
    }

    /* ------------ computations ------------ */

    fn xor<T: ByteIter>(&self, rhs: T) -> ByteArray {
        let ret = ByteArray::from_bytes(
            self.as_slice().iter()
            .zip(rhs.biter())
            .map(|(x, y)| x ^ y)
            .collect::<Vec<_>>());
        assert_eq!(ret.len(), self.len());
        ret
    }
}

/// algorithms for mutable binary data
pub trait MutBinaryAlgo: BinaryAlgo {
    /* ------------ abstract methods ------------ */

    fn as_mut_slice(&mut self) -> &mut [u8];

    /* ------------ computations ------------ */
    fn xor_assign<T: ByteIter>(&mut self, rhs: T) {
        let len = self.len();
        let data = self.as_mut_slice();
        let mut riter = rhs.biter();
        for i in 0..len {
            data[i] ^= riter.next().unwrap();
        }
    }
}

/// byte array that owns the data
pub struct ByteArray {
    data: Vec<u8>
}

/// lightweight ByteArray containing slice of another ByteArray
pub struct ByteArrayView<'a> {
    data: &'a [u8]
}

impl ByteArray {
    /* ------------ constructors ------------ */
    pub fn new() -> Self {
        Self::from_bytes(Vec::new())
    }

    pub fn from_bytes(data: Vec<u8>) -> Self {
        ByteArray { data: data }
    }

    pub fn from_hex(hex: &str) -> Result<ByteArray, CryptoError> {
        match hex.from_hex() {
            Ok(data) => Ok(Self::from_bytes(data)),
            Err(err) => Err(CryptoError::from_msg(
                    format!("failed to convert from hex: {}", err).as_str()))
        }
    }

    pub fn from_base64(b64: &str) -> Result<ByteArray, CryptoError> {
        match b64.from_base64() {
            Ok(data) => Ok(Self::from_bytes(data)),
            Err(err) => Err(CryptoError::from_msg(
                    format!("failed to convert \
                            from base64: {}", err).as_str()))
        }
    }
}

impl BinaryAlgo for ByteArray {
    fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }
}

impl MutBinaryAlgo for ByteArray {
    fn as_mut_slice(&mut self) -> &mut [u8] {
        self.data.as_mut_slice()
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

/// a byte stream
pub trait ByteIter {
    type BIter: Iterator<Item=u8>;

    fn biter(self) -> Self::BIter;
}

impl<'a> ByteIter for &'a ByteArray {
    type BIter = U8IterRefRm<slice::Iter<'a, u8>>;

    fn biter(self) -> Self::BIter {
         U8IterRefRm { iter: self.data.iter() }
    }
}

impl<'a> ByteIter for ByteArrayView<'a> {
    type BIter = U8IterRefRm<slice::Iter<'a, u8>>;

    fn biter(self) -> Self::BIter {
         U8IterRefRm { iter: self.data.iter() }
    }
}

/// repeating a constant and infinite u8 stream
impl ByteIter for u8 {
    type BIter = iter::Repeat<u8>;
    fn biter(self) -> Self::BIter {
        iter::repeat(self)
    }
}

/// make ByteIter from arbitrary iterator
pub struct ByteIterMaker<T> {
    iter: T
}

impl<T> ByteIterMaker<T> {
    pub fn new(iter: T) -> ByteIterMaker<T> {
        ByteIterMaker { iter: iter }
    }
}

impl<T> ByteIter for ByteIterMaker<T> where T: Iterator<Item=u8> {
    type BIter = T;
    fn biter(self) -> Self::BIter {
        self.iter
    }
}

// operator impls
impl<'a, T> BitXor<T> for &'a ByteArray where T: ByteIter {
    type Output = ByteArray;
    fn bitxor(self, rhs: T) -> Self::Output {
        self.xor(rhs)
    }
}

impl<T> BitXorAssign<T> for ByteArray where T: ByteIter {
    fn bitxor_assign(&mut self, rhs: T) {
        self.xor_assign(rhs)
    }
}

