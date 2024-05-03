use crate::cpp_integer::CppUInt64Gadget;
use crate::variable_integer::VariableIntegerGadget;
use bitcoin::consensus::Encodable;
use bitcoin::opcodes::all::{
    OP_PUSHBYTES_1, OP_PUSHBYTES_3, OP_PUSHBYTES_5, OP_PUSHBYTES_8, OP_PUSHBYTES_9,
};
use bitcoin::Amount;
use bitvm::treepp::*;

pub struct TxOutGadget;

impl TxOutGadget {
    pub fn step_1_add_constant_value(value: Amount) -> Script {
        let v = value.to_sat();

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

    pub fn step_1_add_provided_u64() -> Script {
        CppUInt64Gadget::from_u64_in_16bit_limbs()
    }

    pub fn step_2_add_constant_vi(len: usize) -> Script {
        let vi = bitcoin::VarInt::from(len as u64);

        let mut bytes = vec![];
        vi.consensus_encode(&mut bytes).unwrap();

        if vi.size() == 1 {
            Script::from_bytes(vec![OP_PUSHBYTES_1.to_u8(), bytes[0]])
        } else if vi.size() == 3 {
            Script::from_bytes(vec![OP_PUSHBYTES_3.to_u8(), bytes[0], bytes[1], bytes[2]])
        } else if vi.size() == 5 {
            Script::from_bytes(vec![
                OP_PUSHBYTES_5.to_u8(),
                bytes[0],
                bytes[1],
                bytes[2],
                bytes[3],
                bytes[4],
            ])
        } else if vi.size() == 9 {
            Script::from_bytes(vec![
                OP_PUSHBYTES_9.to_u8(),
                bytes[0],
                bytes[1],
                bytes[2],
                bytes[3],
                bytes[4],
                bytes[5],
                bytes[6],
                bytes[7],
                bytes[8],
            ])
        } else {
            unreachable!()
        }
    }

    pub fn step_2_add_provided_small_bitcoin_integer() -> Script {
        VariableIntegerGadget::encode_small_bitcoin_number()
    }
}
