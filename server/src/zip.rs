#![allow(unused)]
//! Module aimed at compression of bytes to shrink network payload

use flate2::Compression;
use flate2::bufread::{ZlibEncoder, ZlibDecoder};
use std::io::prelude::*;

/// Decompressed a byte array
pub fn decode(bytes: &[u8]) -> Vec<u8> {
    let mut e = ZlibDecoder::new(&*bytes);
    let mut decompressed_bytes = Vec::new();
    e.read_to_end(&mut decompressed_bytes).unwrap();
    decompressed_bytes
}

/// Compresses a byte array
pub fn encode(bytes: &[u8]) -> Vec<u8> {
    let mut e = ZlibEncoder::new(&*bytes, Compression::best());
    let mut compressed_bytes = Vec::new();
    e.read_to_end(&mut compressed_bytes).unwrap();
    compressed_bytes
}