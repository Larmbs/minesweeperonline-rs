//! Defines version 2 of the MineSweeper Client Server protocol
#![allow(unused)]

use anyhow::{Context, Result};
use bincode;
use flate2::bufread::{ZlibDecoder, ZlibEncoder};
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;

pub const MAX_BYTES: usize = 10001;
pub type Bytes = Vec<u8>;

#[derive(Serialize, Deserialize, PartialEq)]
pub enum ClientMsg {
    // size: (u16)
    // name: (error_code)
    Error(u16),

    // size: (u16)
    // name: (version)
    // If version is invalid then it throws an error.
    SetVersion(u16),

    // size: (u8, u8, u16)
    // name: (width, height, mine_count)
    // If width or height exceed 100 then throws an error.
    // If mine_count exceeds 100*100 - 1 then throws an error.
    NewGame(u8, u8, u16),

    // size: (u16)
    // name: (index)
    // If index is out of range then throws an error.
    Reveal(u16),

    // size: ()
    // name: ()
    GetTime(),

    // size: ()
    // name: ()
    CloseGame(),
}
impl ClientMsg {
    pub fn to_bytes(&self) -> Result<Bytes> {
        let bytes: Vec<u8> = bincode::serialize(&self)?;
        let mut e = ZlibEncoder::new(&bytes[..], Compression::best());
        let mut compressed_bytes = Vec::new();
        e.read_to_end(&mut compressed_bytes)?;
        Ok(compressed_bytes)
    }
    pub fn from_bytes(bytes: &Bytes) -> Result<Self> {
        let mut e = ZlibDecoder::new(&bytes[..]);
        let mut decompressed_bytes = Vec::new();
        e.read_to_end(&mut decompressed_bytes)?;
        let obj =
            bincode::deserialize(&decompressed_bytes).context("Failed to deserialize message")?;
        Ok(obj)
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum ServerMsg {
    // size: (u16)
    // name: (error_code)
    Error(u16),

    // size: ()
    // name: ()
    Accepted(),

    // size: ([u8; u16])
    // name: ([val; width*height])
    RevealCells(Vec<u8>),

    // size: ([u8; u16])
    // name: ([val; width*height])
    GameWin(Vec<u8>),

    // size: (Vec<u16>)
    // name: (Vec<index>)
    GameLoss(Vec<u16>),

    // size: (String)
    // name: (time)
    Time(String),
}
impl ServerMsg {
    pub fn to_bytes(&self) -> Result<Bytes> {
        let bytes: Vec<u8> = bincode::serialize(&self)?;
        let mut e = ZlibEncoder::new(&bytes[..], Compression::best());
        let mut compressed_bytes = Vec::new();
        e.read_to_end(&mut compressed_bytes)?;
        Ok(compressed_bytes)
    }
    pub fn from_bytes(bytes: &Bytes) -> Result<Self> {
        let mut e = ZlibDecoder::new(&bytes[..]);
        let mut decompressed_bytes = Vec::new();
        e.read_to_end(&mut decompressed_bytes)?;
        let obj =
            bincode::deserialize(&decompressed_bytes).context("Failed to deserialize message")?;
        Ok(obj)
    }
}
