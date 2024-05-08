use crate::structures::hashtype::HashTypeGadget;
use crate::structures::locktime::AbsoluteLockTimeGadget;
use crate::structures::version::VersionGadget;
use bitcoin::absolute::LockTime;
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
        VersionGadget::from_constant(version)
    }

    pub fn step_3_add_provided_nversion() -> Script {
        VersionGadget::from_provided()
    }

    pub fn step_4_add_constant_nlocktime(lock_time: LockTime) -> Script {
        AbsoluteLockTimeGadget::from_constant(lock_time)
    }

    pub fn step_4_add_provided_nlocktime() -> Script {
        AbsoluteLockTimeGadget::from_provided_block_number()
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

    pub fn step_6_sha_outputs() -> Script {
        todo!()
    }

    pub fn step_7_spend_type() -> Script {
        todo!()
    }
}
