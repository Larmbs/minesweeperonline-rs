#![allow(unused)]
use anyhow::Context;
use bincode;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Serialize, PartialEq)]
pub enum MsgSend {
    // (message)
    Error(String),
    // ((width, height), mine_count)
    Connect((usize, usize), usize),
    // (index)
    Reveal(usize),
}
impl TryInto<Vec<u8>> for MsgSend {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<Vec<u8>, <MsgSend as TryInto<Vec<u8>>>::Error> {
        bincode::serialize(&self).context("Failed to serialize the message")
    }
}

#[derive(Deserialize, PartialEq)]
pub enum MsgReceive {
    // (message)
    Error(String),
    // ()
    ConnectionAccepted,
    // ([index, value])
    RevealCells(Vec<(usize, u8)>),
    // (win/loss, time, [index])
    GameEnd(bool, String, Vec<usize>),
}
impl TryFrom<Vec<u8>> for MsgReceive {
    type Error = anyhow::Error;

    fn try_from(value: Vec<u8>) -> anyhow::Result<Self, <MsgReceive as TryFrom<Vec<u8>>>::Error> {
        bincode::deserialize(&value).context("Failed to deserialize the message")
    }
}

/// Sends a message, panics if it fails.
pub fn send(socket: &mut TcpStream, msg: MsgSend) -> MsgReceive {
    let bytes: Vec<u8> = msg.try_into().unwrap();
    socket.write_all(&bytes).unwrap();

    let mut buffer = vec![0u8; 1024];
    socket.read(&mut buffer).unwrap();
    MsgReceive::try_from(buffer).unwrap()
}

/// Tries to send a message, returns an error if it fails.
pub fn try_send(socket: &mut TcpStream, msg: MsgSend) -> anyhow::Result<MsgReceive> {
    let bytes: Vec<u8> = msg.try_into()?;
    socket.write_all(&bytes)?;

    let mut buffer = vec![0u8; 1024];
    socket.read(&mut buffer)?;
    MsgReceive::try_from(buffer)
}
