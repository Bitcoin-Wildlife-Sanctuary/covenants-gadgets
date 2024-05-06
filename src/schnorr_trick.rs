use crate::internal_structures::cpp_int_32::CppInt32Gadget;
use crate::structures::hashtype::HashTypeGadget;
use bitcoin::absolute::LockTime;
use bitcoin::opcodes::all::OP_PUSHBYTES_4;
use bitcoin::transaction::Version;
use bitcoin::TapSighashType;
use bitvm::treepp::*;

pub struct SchnorrTrickGadget;

impl SchnorrTrickGadget {
    pub fn step1_add_epoch() -> Script {
        script! {
            OP_PUSHBYTES_1 OP_PUSHBYTES_0
        }
    }

    pub fn step_2_add_constant_hash_type(hash_type: TapSighashType) -> Script {
        HashTypeGadget::from_constant(hash_type)
    }

    pub fn step_2_check_provided_hash_type() -> Script {
        HashTypeGadget::from_provided()
    }

    pub fn step_3_add_constant_nversion(version: Version) -> Script {
        assert!(version == Version::ONE || version == Version::TWO);

        if version == Version::ONE {
            script! {
                OP_PUSHBYTES_4 OP_PUSHBYTES_1 OP_PUSHBYTES_0 OP_PUSHBYTES_0 OP_PUSHBYTES_0
            }
        } else {
            script! {
                OP_PUSHBYTES_4 OP_PUSHBYTES_2 OP_PUSHBYTES_0 OP_PUSHBYTES_0 OP_PUSHBYTES_0
            }
        }
    }

    pub fn step_3_add_provided_nversion() -> Script {
        // input: nversion (as a bitcoin integer 1 or 2)

        script! {
            OP_DUP 1 OP_EQUAL OP_IF
                OP_DROP
                OP_PUSHBYTES_4 OP_PUSHBYTES_1 OP_PUSHBYTES_0 OP_PUSHBYTES_0 OP_PUSHBYTES_0
            OP_ELSE
                2 OP_EQUAL OP_VERIFY
                OP_PUSHBYTES_4 OP_PUSHBYTES_2 OP_PUSHBYTES_0 OP_PUSHBYTES_0 OP_PUSHBYTES_0
            OP_ENDIF
        }
    }

    pub fn step_4_add_constant_nlocktime(lock_time: LockTime) -> Script {
        let v = lock_time.to_consensus_u32();
        Script::from_bytes(vec![
            OP_PUSHBYTES_4.to_u8(),
            (v & 0xff) as u8,
            ((v >> 8) & 0xff) as u8,
            ((v >> 16) & 0xff) as u8,
            ((v >> 24) & 0xff) as u8,
        ])
    }

    pub fn step_4_add_provided_nlocktime() -> Script {
        // input: nlocktime (as a bitcoin integer)

        script! {
            { CppInt32Gadget::from_bitcoin_integer() }
        }
    }

    pub fn step_5_cat_all_input_info() -> Script {
        // stack elements:
        //   sha_prevouts
        //   sha_amounts
        //   sha_scriptpubkeys
        //   sha_sequences
        script! {
            OP_CAT OP_CAT OP_CAT
        }
    }
}
