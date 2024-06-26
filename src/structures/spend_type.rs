use crate::treepp::*;

/// Gadget for the spend type.
pub struct SpendTypeGadget;

impl SpendTypeGadget {
    /// Construct the spend type using the constant extension flag and whether the annex is present.
    pub fn from_constant(ext_flag: u8, has_annex: bool) -> Script {
        // other ext flags can be very tricky, as one cannot easily represent a number larger than
        // 127 using a single byte in the stack without a lot of manual conversions.
        assert!(ext_flag == 0 || ext_flag == 1);

        // annex has not been used in the mainnet.
        assert_eq!(has_annex, false);

        if ext_flag == 0 {
            script! {
                OP_PUSHBYTES_1 OP_PUSHBYTES_0
            }
        } else {
            script! {
                { 2 }
            }
        }
    }

    /// Construct the spend type from the provided type on the stack.
    ///
    /// It verifies that the provided spend type is either 0 or 1.
    pub fn from_provided() -> Script {
        script! {
            OP_DUP 0 OP_EQUAL OP_IF
                OP_PUSHBYTES_1 OP_PUSHBYTES_0
            OP_ELSE
                OP_DUP 1 OP_EQUALVERIFY
                OP_PUSHBYTES_1 OP_PUSHBYTES_1
            OP_ENDIF
        }
    }
}
