#![allow(dead_code)]
pub(crate) const SIMPLE_STRING: u8 = b'+';
pub(crate) const ERROR: u8 = b'-';
pub(crate) const INTEGER: u8 = b':';
pub(crate) const BULK_STRING: u8 = b'$';
pub(crate) const ARRAY: u8 = b'*';
pub(crate) const CRLN: &[u8; 2] = b"\r\n";
