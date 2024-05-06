#![allow(non_snake_case)]

use bitcoin::opcodes::all::OP_CAT;
use bitcoin::Opcode;
use bitvm::treepp::*;

pub const OP_CAT2: Opcode = OP_CAT;

pub fn OP_CAT3() -> Script {
    script! {
        OP_CAT OP_CAT
    }
}

pub fn OP_CAT4() -> Script {
    script! {
        OP_CAT OP_CAT OP_CAT
    }
}

pub fn OP_CAT5() -> Script {
    script! {
        OP_CAT OP_CAT OP_CAT OP_CAT
    }
}

pub fn OP_CAT6() -> Script {
    script! {
        OP_CAT OP_CAT OP_CAT OP_CAT OP_CAT
    }
}
