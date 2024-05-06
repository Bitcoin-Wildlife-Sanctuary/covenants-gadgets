use bitcoin::consensus::Encodable;
use bitcoin::opcodes::all::{OP_PUSHBYTES_1, OP_PUSHBYTES_3, OP_PUSHBYTES_5, OP_PUSHBYTES_9};
use bitvm::treepp::*;

pub struct VariableLengthIntegerGadget;

impl VariableLengthIntegerGadget {
    pub fn from_small_bitcoin_number() -> Script {
        script! {
            // making sure the number is smaller than 128
            OP_DUP
            { 128 } OP_LESSTHAN OP_VERIFY
        }
    }

    pub fn from_constant(vi: usize) -> Script {
        let vi = bitcoin::VarInt::from(vi as u64);

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
}
