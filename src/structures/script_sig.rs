use crate::treepp::*;

/// Gadget for the script signature.
///
/// In segwit, it is always empty.
pub struct ScriptSigGadget;

impl ScriptSigGadget {
    /// Construct the script signature using the default value (empty) for segwit.
    pub fn segregated_witness() -> Script {
        script! {
            OP_PUSHBYTES_1 OP_PUSHBYTES_0
        }
    }
}
