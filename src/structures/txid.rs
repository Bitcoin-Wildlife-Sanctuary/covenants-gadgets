use bitcoin::secp256k1::ThirtyTwoByteHash;
use bitcoin::Txid;
use bitvm::treepp::*;

/// Gadget for the transaction ID (txid).
pub struct TxIdGadget;

impl TxIdGadget {
    /// Construct the transaction ID from constant data.
    pub fn from_constant(txid: Txid) -> Script {
        script! {
            { txid.as_raw_hash().into_32().to_vec() }
        }
    }
}
