use crate::treepp::*;
use crate::utils::pseudo::{OP_CAT_BACKUP, OP_CAT_HASH, OP_CAT_RESTORE, OP_HINT};

/// Gadget for stackhash
pub struct StackHash;

impl StackHash {
    /// cat + hash stack elements one-by-one, outputs only one hash value
    /// fixed depth, say 3
    pub fn hash3() -> Script {
        script! {
            // initial hash
            OP_DEPTH
            OP_SHA256
            // concate and hash
            OP_DEPTH 1 OP_GREATERTHAN OP_IF
                OP_CAT_HASH
            OP_ENDIF
            OP_DEPTH 1 OP_GREATERTHAN OP_IF
                OP_CAT_HASH
            OP_ENDIF
            OP_DEPTH 1 OP_GREATERTHAN OP_IF
                OP_CAT_HASH
            OP_ENDIF
        }
    }
    /// cat + hash stack elements one-by-one, outputs the final hash value and its original elements
    /// where the hash value is on the top, similarly the stack height is fixed, say 3
    pub fn hash3_hints() -> Script {
        script! {
            // initial hash
            OP_DEPTH
            OP_SHA256
            // concat and hash, before that we need to backup stack elements
            OP_DEPTH 1 OP_GREATERTHAN OP_IF
                OP_CAT_BACKUP OP_CAT_HASH
            OP_ENDIF
            OP_DEPTH 1 OP_GREATERTHAN OP_IF
                OP_CAT_BACKUP OP_CAT_HASH
            OP_ENDIF
            OP_DEPTH 1 OP_GREATERTHAN OP_IF
                OP_CAT_BACKUP OP_CAT_HASH
            OP_ENDIF
            // restore original elements
            OP_CAT_RESTORE
            OP_CAT_RESTORE
            OP_CAT_RESTORE
            // drop intial hash
            OP_DROP
            // roll final hash on the top
            OP_HINT
        }
    }
}
