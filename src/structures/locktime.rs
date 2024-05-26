use crate::internal_structures::cpp_int_32::CppInt32Gadget;
use crate::treepp::*;
use crate::utils::push_u32_4bytes;
use bitcoin::{absolute, relative};

/// Gadget for the transaction's locktime.
pub struct LockTimeGadget;

impl LockTimeGadget {
    /// Construct the locktime using a constant absolute locktime.
    pub fn from_constant_absolute(v: &absolute::LockTime) -> Script {
        let v = v.to_consensus_u32();
        push_u32_4bytes(v)
    }

    /// Construct the locktime using a constant relative locktime.
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

    /// Construct the locktime using the provided unix timestamp.
    ///
    /// It checks if the provided value is greater or equal to 500000000,
    /// which is a requirement for the unix timestamp provided here.
    pub fn from_provided_unix_timestamp() -> Script {
        script! {
            OP_DUP { 500000000 } OP_GREATERTHANOREQUAL OP_VERIFY
            { CppInt32Gadget::from_bitcoin_integer() }
        }
    }

    /// Construct the locktime using the provided block number.
    ///
    /// It checks if the provided value is smaller than 500000000,
    /// which is a requirement for the block number provided here.
    pub fn from_provided_block_number() -> Script {
        script! {
            OP_DUP { 500000000 } OP_LESSTHAN OP_VERIFY
            { CppInt32Gadget::from_positive_bitcoin_integer() }
        }
    }
}
