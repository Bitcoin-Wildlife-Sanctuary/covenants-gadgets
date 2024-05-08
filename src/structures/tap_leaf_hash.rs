use bitcoin::secp256k1::ThirtyTwoByteHash;
use bitcoin::TapLeafHash;
use bitvm::treepp::*;

pub struct TapLeafHashGadget;

impl TapLeafHashGadget {
    pub fn from_constant(tap_leaf_hash: &TapLeafHash) -> Script {
        script! {
            { tap_leaf_hash.as_raw_hash().into_32().to_vec() }
        }
    }
}
