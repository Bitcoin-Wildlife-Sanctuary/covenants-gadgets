use bitcoin::secp256k1::ThirtyTwoByteHash;
use bitcoin::Txid;
use bitvm::treepp::*;

pub struct TxIdGadget;

impl TxIdGadget {
    pub fn from_constant(txid: Txid) -> Script {
        script! {
            { txid.as_raw_hash().into_32().to_vec() }
        }
    }
}
