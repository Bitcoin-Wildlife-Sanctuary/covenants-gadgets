#![allow(non_snake_case)]

use crate::treepp::*;

/// Doing OP_CAT one time to concatenate 2 elements together.
pub fn OP_CAT2() -> Script {
    script! {
        OP_CAT
    }
}

/// Doing OP_CAT 2 times to concatenate 3 elements together.
pub fn OP_CAT3() -> Script {
    script! {
        OP_CAT OP_CAT
    }
}

/// Doing OP_CAT 3 times to concatenate 4 elements together.
pub fn OP_CAT4() -> Script {
    script! {
        OP_CAT OP_CAT OP_CAT
    }
}

/// Doing OP_CAT 4 times to concatenate 5 elements together.
pub fn OP_CAT5() -> Script {
    script! {
        OP_CAT OP_CAT OP_CAT OP_CAT
    }
}

/// Doing OP_CAT 5 times to concatenate 6 elements together.
pub fn OP_CAT6() -> Script {
    script! {
        OP_CAT OP_CAT OP_CAT OP_CAT OP_CAT
    }
}
