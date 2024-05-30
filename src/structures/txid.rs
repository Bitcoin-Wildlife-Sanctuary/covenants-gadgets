use crate::treepp::*;
use bitcoin::Txid;

/// Gadget for the transaction ID (txid).
pub struct TxIdGadget;

impl TxIdGadget {
    /// Construct the transaction ID from constant data.
    pub fn from_constant(txid: Txid) -> Script {
        script! {
            { AsRef::<[u8]>::as_ref(&txid).to_vec() }
        }
    }

    /// Construct the transaction ID from provided data of 32 bytes.
    pub fn from_provided() -> Script {
        script! {
            OP_SIZE 32 OP_EQUALVERIFY
        }
    }
}
