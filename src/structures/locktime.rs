use crate::internal_structures::cpp_int_32::CppInt32Gadget;
use crate::utils::push_u32_4bytes;
use bitcoin::relative::LockTime;
use bitcoin::{absolute, relative};
use bitvm::treepp::*;

pub struct AbsoluteLockTimeGadget;

impl AbsoluteLockTimeGadget {
    pub fn from_constant(v: absolute::LockTime) -> Script {
        let v = v.to_consensus_u32();
        push_u32_4bytes(v)
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

pub struct RelativeLockTimeGadget;

impl RelativeLockTimeGadget {
    pub fn from_constant(v: relative::LockTime) -> Script {
        match v {
            LockTime::Blocks(v) => {
                let v = v.to_consensus_u32();
                push_u32_4bytes(v)
            }
            LockTime::Time(v) => {
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
