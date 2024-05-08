#![allow(non_snake_case)]

use bitvm::treepp::*;

pub fn OP_CAT2() -> Script {
    script! {
        OP_CAT
    }
}

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
