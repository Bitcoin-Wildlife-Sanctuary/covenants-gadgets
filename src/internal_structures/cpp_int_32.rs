use bitvm::treepp::*;

pub struct CppInt32Gadget;

impl CppInt32Gadget {
    pub fn from_bitcoin_integer() -> Script {
        script! {
            OP_DUP OP_ABS
            // stack: x abs(x)

            OP_SIZE 4 OP_LESSTHAN
            OP_IF
                // stack: abs(x) x abs(x)
                OP_DUP OP_ROT
                OP_EQUAL OP_TOALTSTACK

                // stack: abs(a), altstack: is_positive
                OP_SIZE 2 OP_LESSTHAN OP_IF OP_PUSHBYTES_2 OP_PUSHBYTES_0 OP_PUSHBYTES_0 OP_CAT OP_ENDIF
                OP_SIZE 3 OP_LESSTHAN OP_IF OP_PUSHBYTES_1 OP_PUSHBYTES_0 OP_CAT OP_ENDIF

                OP_FROMALTSTACK
                OP_IF
                    OP_PUSHBYTES_1 OP_PUSHBYTES_0
                OP_ELSE
                    OP_PUSHBYTES_1 OP_LEFT
                OP_ENDIF
                OP_CAT
            OP_ELSE
                OP_DROP // abs doesn't change the number of bytes of the representation
            OP_ENDIF
        }
    }

    pub fn from_positive_bitcoin_integer() -> Script {
        script! {
            OP_SIZE 3 OP_LESSTHAN OP_IF OP_PUSHBYTES_2 OP_PUSHBYTES_0 OP_PUSHBYTES_0 OP_CAT OP_ENDIF
            OP_SIZE 4 OP_LESSTHAN OP_IF OP_PUSHBYTES_1 OP_PUSHBYTES_0 OP_CAT OP_ENDIF
            OP_SIZE 4 OP_LESSTHAN OP_IF OP_PUSHBYTES_1 OP_PUSHBYTES_0 OP_CAT OP_ENDIF
        }
    }
}
