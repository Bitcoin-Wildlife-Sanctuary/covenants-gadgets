use crate::treepp::*;
use bitcoin::TapLeafHash;

/// Gadget for tap leaf hash.
pub struct TapLeafHashGadget;

impl TapLeafHashGadget {
    /// Construct the tap leaf hash from constant data.
    pub fn from_constant(tap_leaf_hash: &TapLeafHash) -> Script {
        script! {
            { AsRef::<[u8]>::as_ref(&tap_leaf_hash).to_vec() }
        }
    }
}
