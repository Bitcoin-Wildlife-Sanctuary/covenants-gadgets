use bitcoin::opcodes::all::OP_PUSHBYTES_8;
use bitcoin::{Amount, ScriptBuf};

pub struct TxOutGadget;

impl TxOutGadget {
    pub fn step_1_add_constant_value(value: Amount) -> ScriptBuf {
        let v = value.to_sat();

        ScriptBuf::from_bytes(vec![
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

    pub fn step_1_add_provided_value() {}
}
