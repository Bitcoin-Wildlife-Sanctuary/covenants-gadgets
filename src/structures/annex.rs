use bitcoin::consensus::Encodable;
use bitcoin::sighash::Annex;
use bitvm::treepp::*;

pub struct AnnexGadget;

impl AnnexGadget {
    pub fn none() -> Script {
        script! {
            OP_PUSHBYTES_0
        }
    }

    pub fn from_constant(annex: &Annex) -> Script {
        let mut bytes = vec![];
        annex.consensus_encode(&mut bytes).unwrap();

        script! {
            { bytes }
        }
    }
}
