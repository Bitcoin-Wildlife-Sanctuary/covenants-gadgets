use crate::internal_structures::cpp_int_32::CppInt32Gadget;
use bitcoin::opcodes::all::OP_PUSHBYTES_4;
use bitvm::treepp::*;

/// Gadget for the code separator position.
pub struct CodeSepPosGadget;

impl CodeSepPosGadget {
    /// Construct a dummy code separator position 0xffffffff indicating no such
    /// separator has ever been encountered.
    pub fn no_code_sep_executed() -> Script {
        Script::from_bytes(vec![OP_PUSHBYTES_4.to_u8(), 0xFF, 0xFF, 0xFF, 0xFF])
    }

    /// Construct the code separator position from constant data.
    pub fn from_constant(code_sep_pos: u32) -> Script {
        CppInt32Gadget::from_constant(code_sep_pos)
    }

    /// Construct the code separator position from a Bitcoin integer on the stack.
    pub fn from_bitcoin_integer() -> Script {
        CppInt32Gadget::from_bitcoin_integer()
    }

    /// Construct the code separator position from a positive Bitcoin integer on the stack.
    ///
    /// This is faster than `from_bitcoin_integer` since it doesn't handle negative numbers.
    pub fn from_positive_bitcoin_integer() -> Script {
        CppInt32Gadget::from_positive_bitcoin_integer()
    }
}
