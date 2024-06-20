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

/// Pull one element from the hints.
pub fn OP_HINT() -> Script {
    script! {
        OP_DEPTH OP_1SUB OP_ROLL
    }
}

/// Backup the top two elements before OP_CAT_HASH
pub fn OP_CAT_BACKUP() -> Script {
    script! {
        OP_2DUP OP_TOALTSTACK OP_TOALTSTACK
    }
}

/// Cat and Hash
pub fn OP_CAT_HASH() -> Script {
    script! {
        OP_CAT OP_SHA256
    }
}

/// Restore after OP_CAT_HASH
pub fn OP_CAT_RESTORE() -> Script {
    script! {
        OP_FROMALTSTACK OP_FROMALTSTACK
    }
}
