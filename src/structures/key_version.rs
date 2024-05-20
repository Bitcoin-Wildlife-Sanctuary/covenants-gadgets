use bitvm::treepp::*;

/// Gadget for the key version used in the taproot CheckSigVerify extension.
pub struct KeyVersionGadget;

impl KeyVersionGadget {
    /// Construct the key version from constant data.
    ///
    /// The only valid version number for now is 0.
    pub fn from_constant(version: u8) -> Script {
        assert_eq!(version, 0);

        script! {
            OP_PUSHBYTES_1 OP_PUSHBYTES_0
        }
    }
}
