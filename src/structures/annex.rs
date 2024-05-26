use crate::treepp::*;
use bitcoin::consensus::Encodable;
use bitcoin::sighash::Annex;

/// Gadget for the annex (currently unused).
pub struct AnnexGadget;

impl AnnexGadget {
    /// Construct an empty annex (which is the only supported format for now).
    pub fn none() -> Script {
        script! {
            OP_PUSHBYTES_0
        }
    }

    /// Construct the annex from constant data.
    pub fn from_constant(annex: &Annex) -> Script {
        let mut bytes = vec![];
        annex.consensus_encode(&mut bytes).unwrap();

        script! {
            { bytes }
        }
    }
}
