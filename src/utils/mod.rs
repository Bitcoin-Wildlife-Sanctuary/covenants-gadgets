use bitcoin::opcodes::all::{OP_PUSHBYTES_4, OP_PUSHBYTES_8};
use bitvm::treepp::*;

pub mod pseudo;

pub fn push_u32_4bytes(v: u32) -> Script {
    Script::from_bytes(vec![
        OP_PUSHBYTES_4.to_u8(),
        (v & 0xff) as u8,
        ((v >> 8) & 0xff) as u8,
        ((v >> 16) & 0xff) as u8,
        ((v >> 24) & 0xff) as u8,
    ])
}

pub fn push_u64_8bytes(v: u64) -> Script {
    Script::from_bytes(vec![
        OP_PUSHBYTES_8.to_u8(),
        (v & 0xff) as u8,
        ((v >> 8) & 0xff) as u8,
        ((v >> 16) & 0xff) as u8,
        ((v >> 24) & 0xff) as u8,
        ((v >> 32) & 0xff) as u8,
        ((v >> 40) & 0xff) as u8,
        ((v >> 48) & 0xff) as u8,
        ((v >> 56) & 0xff) as u8,
    ])
}
