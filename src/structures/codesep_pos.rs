use crate::internal_structures::cpp_int_32::CppInt32Gadget;
use bitcoin::opcodes::all::OP_PUSHBYTES_4;
use bitvm::treepp::*;

pub struct CodeSepPosGadget;

impl CodeSepPosGadget {
    pub fn no_code_sep_executed() -> Script {
        Script::from_bytes(vec![OP_PUSHBYTES_4.to_u8(), 0xFF, 0xFF, 0xFF, 0xFF])
    }

    pub fn from_constant(code_sep_pos: u32) -> Script {
        CppInt32Gadget::from_constant(code_sep_pos)
    }

    pub fn from_bitcoin_integer() -> Script {
        CppInt32Gadget::from_bitcoin_integer()
    }

    pub fn from_positive_bitcoin_integer() -> Script {
        CppInt32Gadget::from_positive_bitcoin_integer()
    }
}
