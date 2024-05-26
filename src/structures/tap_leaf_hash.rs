use crate::treepp::*;
use bitcoin::secp256k1::ThirtyTwoByteHash;
use bitcoin::TapLeafHash;

/// Gadget for tap leaf hash.
pub struct TapLeafHashGadget;

impl TapLeafHashGadget {
    /// Construct the tap leaf hash from constant data.
    pub fn from_constant(tap_leaf_hash: &TapLeafHash) -> Script {
        script! {
            { tap_leaf_hash.as_raw_hash().into_32().to_vec() }
        }
    }
}
