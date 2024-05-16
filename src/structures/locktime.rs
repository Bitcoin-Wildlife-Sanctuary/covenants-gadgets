use crate::internal_structures::cpp_int_32::CppInt32Gadget;
use crate::utils::push_u32_4bytes;
use bitcoin::{absolute, relative};
use bitvm::treepp::*;

pub struct LockTimeGadget;

impl LockTimeGadget {
    pub fn from_constant_absolute(v: &absolute::LockTime) -> Script {
        let v = v.to_consensus_u32();
        push_u32_4bytes(v)
    }

    pub fn from_constant_relative(v: &relative::LockTime) -> Script {
        match v {
            relative::LockTime::Blocks(v) => {
                let v = v.to_consensus_u32();
                push_u32_4bytes(v)
            }
            relative::LockTime::Time(v) => {
                let v = v.to_consensus_u32();
                push_u32_4bytes(v)
            }
        }
    }

    pub fn from_provided_unix_timestamp() -> Script {
        script! {
            OP_DUP { 500000000 } OP_GREATERTHANOREQUAL OP_VERIFY
            { CppInt32Gadget::from_bitcoin_integer() }
        }
    }

    pub fn from_provided_block_number() -> Script {
        script! {
            OP_DUP { 500000000 } OP_LESSTHAN OP_VERIFY
            { CppInt32Gadget::from_positive_bitcoin_integer() }
        }
    }
}
