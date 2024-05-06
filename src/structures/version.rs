use bitcoin::blockdata::transaction::Version;
use bitvm::treepp::*;
use crate::utils::push_u32_4bytes;

pub struct VersionGadget;

impl VersionGadget {
    pub fn from_constant(version: Version) -> Script {
        match version {
            Version(v) => push_u32_4bytes(v as u32)
        }
    }
}