use anyhow::Context;
use bincode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, PartialEq)]
pub enum MsgReceive {
    // (message)
    Error(String),
    // ((width, height), mine_count)
    Connect((usize, usize), usize),
    // (index)
    Reveal(usize),
}
impl TryFrom<&Vec<u8>> for MsgReceive {
    type Error = anyhow::Error;

    fn try_from(value: &Vec<u8>) -> anyhow::Result<Self, <MsgReceive as TryFrom<&Vec<u8>>>::Error> {
        bincode::deserialize(&value).context("Failed to deserialize the message")
    }
}

#[derive(Serialize, PartialEq)]
pub enum MsgSend {
    // (message)
    Error(String),
    // ()
    ConnectionAccepted,
    // ([index, value])
    RevealCells(Vec<(usize, u8)>),
    // (time, [index, value])
    GameWin(String, Vec<(usize, u8)>),
    // (time, [index])
    GameLoss(String, Vec<usize>),
}
impl TryInto<Vec<u8>> for MsgSend {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<Vec<u8>, <MsgSend as TryInto<Vec<u8>>>::Error> {
        bincode::serialize(&self).context("Failed to serialize the message")
    }
}
