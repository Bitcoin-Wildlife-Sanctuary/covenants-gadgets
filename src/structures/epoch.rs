use bitvm::treepp::*;

/// Gadget for the epoch, the leading byte 0x00 in the taproot CheckSigVerify.
pub struct EpochGadget;

impl EpochGadget {
    /// Construct the epoch using the default value (0).
    pub fn default() -> Script {
        script! {
            OP_PUSHBYTES_1 OP_PUSHBYTES_0
        }
    }
}
