use crate::treepp::*;
use crate::utils::push_u32_4bytes;
use bitcoin::blockdata::transaction::Version;

/// Gadget for the transaction version number.
pub struct VersionGadget;

impl VersionGadget {
    /// Construct the version from constant data.
    pub fn from_constant(version: &Version) -> Script {
        match version {
            Version(v) => push_u32_4bytes(*v as u32),
        }
    }

    /// Construct the version from the data in the stack, verifying that
    /// the data is either 1 or 2.
    pub fn from_provided() -> Script {
        script! {
            OP_DUP 1 OP_EQUAL
            OP_OVER 2 OP_EQUAL OP_BOOLOR OP_VERIFY
            OP_PUSHBYTES_3 OP_PUSHBYTES_0 OP_PUSHBYTES_0 OP_PUSHBYTES_0
            OP_CAT
        }
    }
}
