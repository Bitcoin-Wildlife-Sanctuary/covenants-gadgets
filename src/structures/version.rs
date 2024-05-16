use crate::utils::push_u32_4bytes;
use bitcoin::blockdata::transaction::Version;
use bitvm::treepp::*;

pub struct VersionGadget;

impl VersionGadget {
    pub fn from_constant(version: &Version) -> Script {
        match version {
            Version(v) => push_u32_4bytes(*v as u32),
        }
    }

    pub fn from_provided() -> Script {
        script! {
            OP_DUP 1 OP_EQUAL
            OP_OVER 2 OP_EQUAL OP_BOOLOR OP_VERIFY
            OP_PUSHBYTES_3 OP_PUSHBYTES_0 OP_PUSHBYTES_0 OP_PUSHBYTES_0
            OP_CAT
        }
    }
}
