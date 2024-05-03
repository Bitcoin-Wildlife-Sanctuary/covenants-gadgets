use bitvm::pseudo::OP_256MUL;
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

pub struct CppUInt64Gadget;

impl CppUInt64Gadget {
    pub fn from_bitcoin_integer() -> Script {
        script! {
            // sanity check: input must be greater or equal to zero
            OP_DUP 0 OP_GREATERTHANOREQUAL OP_VERIFY

            // pad it to 8 bytes
            OP_SIZE 5 OP_LESSTHAN OP_IF OP_PUSHBYTES_4 OP_PUSHBYTES_0 OP_PUSHBYTES_0 OP_PUSHBYTES_0 OP_PUSHBYTES_0 OP_CAT OP_ENDIF
            OP_SIZE 7 OP_LESSTHAN OP_IF OP_PUSHBYTES_2 OP_PUSHBYTES_0 OP_PUSHBYTES_0 OP_CAT OP_ENDIF
            OP_SIZE 8 OP_LESSTHAN OP_IF OP_PUSHBYTES_1 OP_PUSHBYTES_0 OP_CAT OP_ENDIF
            OP_SIZE 8 OP_LESSTHAN OP_IF OP_PUSHBYTES_1 OP_PUSHBYTES_0 OP_CAT OP_ENDIF
        }
    }

    pub fn from_two_bitcoin_integers() -> Script {
        script! {
            { CppInt32Gadget::from_bitcoin_integer() }
            OP_SWAP
            { CppInt32Gadget::from_bitcoin_integer() }
            OP_SWAP OP_CAT
        }
    }

    pub fn from_u64_in_16bit_limbs() -> Script {
        script! {
            OP_SWAP OP_DUP 32768 OP_GREATERTHANOREQUAL OP_IF
                32768 OP_SUB { 1 }
            OP_ELSE
                { 0 }
            OP_ENDIF
            OP_TOALTSTACK
            OP_256MUL
            OP_256MUL
            OP_ADD

            OP_SIZE 2 OP_LESSTHAN OP_IF OP_PUSHBYTES_2 OP_PUSHBYTES_0 OP_PUSHBYTES_0 OP_CAT OP_ENDIF
            OP_SIZE 3 OP_LESSTHAN OP_IF OP_PUSHBYTES_1 OP_PUSHBYTES_0 OP_CAT OP_ENDIF
            OP_SIZE 3 OP_LESSTHAN OP_IF OP_PUSHBYTES_1 OP_PUSHBYTES_0 OP_CAT OP_ENDIF

            OP_FROMALTSTACK
            OP_IF
                OP_SIZE 4 OP_LESSTHAN OP_IF OP_PUSHBYTES_1 OP_LEFT OP_CAT
                OP_ELSE OP_NEGATE OP_ENDIF
            OP_ELSE
                OP_SIZE 4 OP_LESSTHAN OP_IF OP_PUSHBYTES_1 OP_PUSHBYTES_0 OP_CAT OP_ENDIF
            OP_ENDIF

            OP_TOALTSTACK

            OP_SWAP OP_DUP 32768 OP_GREATERTHANOREQUAL OP_IF
                32768 OP_SUB { 1 }
            OP_ELSE
                { 0 }
            OP_ENDIF
            OP_TOALTSTACK
            OP_256MUL
            OP_256MUL
            OP_ADD

            OP_SIZE 2 OP_LESSTHAN OP_IF OP_PUSHBYTES_2 OP_PUSHBYTES_0 OP_PUSHBYTES_0 OP_CAT OP_ENDIF
            OP_SIZE 3 OP_LESSTHAN OP_IF OP_PUSHBYTES_1 OP_PUSHBYTES_0 OP_CAT OP_ENDIF
            OP_SIZE 3 OP_LESSTHAN OP_IF OP_PUSHBYTES_1 OP_PUSHBYTES_0 OP_CAT OP_ENDIF

            OP_FROMALTSTACK
            OP_IF
                OP_SIZE 4 OP_LESSTHAN OP_IF OP_PUSHBYTES_1 OP_LEFT OP_CAT
                OP_ELSE OP_NEGATE OP_ENDIF
            OP_ELSE
                OP_SIZE 4 OP_LESSTHAN OP_IF OP_PUSHBYTES_1 OP_PUSHBYTES_0 OP_CAT OP_ENDIF
            OP_ENDIF

            OP_FROMALTSTACK
            OP_SWAP
            OP_CAT
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cpp_integer::{CppInt32Gadget, CppUInt64Gadget};
    use bitvm::bigint::U64;
    use bitvm::treepp::*;

    #[test]
    fn test_cpp_int32_from_bitcoin_integer() {
        let v = 0x12;
        let script = script! {
            { v }
            { CppInt32Gadget::from_bitcoin_integer() }
            OP_PUSHBYTES_4 OP_PUSHBYTES_18 OP_PUSHBYTES_0 OP_PUSHBYTES_0 OP_PUSHBYTES_0
            OP_EQUAL
        };
        let res = execute_script(script);
        assert!(res.success);

        let v = 0x1234;
        let script = script! {
            { v }
            { CppInt32Gadget::from_bitcoin_integer() }
            OP_PUSHBYTES_4 OP_PUSHBYTES_52 OP_PUSHBYTES_18 OP_PUSHBYTES_0 OP_PUSHBYTES_0
            OP_EQUAL
        };
        let res = execute_script(script);
        assert!(res.success);

        let v = 0x123456;
        let script = script! {
            { v }
            { CppInt32Gadget::from_bitcoin_integer() }
            OP_PUSHBYTES_4 OP_PUSHNUM_6 OP_PUSHBYTES_52 OP_PUSHBYTES_18 OP_PUSHBYTES_0
            OP_EQUAL
        };
        let res = execute_script(script);
        assert!(res.success);

        let v = 0x12345678;
        let script = script! {
            { v }
            { CppInt32Gadget::from_bitcoin_integer() }
            OP_PUSHBYTES_4 OP_OVER OP_PUSHNUM_6 OP_PUSHBYTES_52 OP_PUSHBYTES_18
            OP_EQUAL
        };
        let res = execute_script(script);
        assert!(res.success);

        let v = 0x78345612;
        let script = script! {
            { v }
            { CppInt32Gadget::from_bitcoin_integer() }
            OP_PUSHBYTES_4 OP_PUSHBYTES_18 OP_PUSHNUM_6 OP_PUSHBYTES_52 OP_OVER
            OP_EQUAL
        };
        let res = execute_script(script);
        assert!(res.success);
    }

    #[test]
    fn test_cpp_int32_from_positive_bitcoin_integer() {
        let v = 0x12;
        let script = script! {
            { v }
            { CppInt32Gadget::from_positive_bitcoin_integer() }
            OP_PUSHBYTES_4 OP_PUSHBYTES_18 OP_PUSHBYTES_0 OP_PUSHBYTES_0 OP_PUSHBYTES_0
            OP_EQUAL
        };
        let res = execute_script(script);
        assert!(res.success);

        let v = 0x1234;
        let script = script! {
            { v }
            { CppInt32Gadget::from_positive_bitcoin_integer() }
            OP_PUSHBYTES_4 OP_PUSHBYTES_52 OP_PUSHBYTES_18 OP_PUSHBYTES_0 OP_PUSHBYTES_0
            OP_EQUAL
        };
        let res = execute_script(script);
        assert!(res.success);

        let v = 0x123456;
        let script = script! {
            { v }
            { CppInt32Gadget::from_positive_bitcoin_integer() }
            OP_PUSHBYTES_4 OP_PUSHNUM_6 OP_PUSHBYTES_52 OP_PUSHBYTES_18 OP_PUSHBYTES_0
            OP_EQUAL
        };
        let res = execute_script(script);
        assert!(res.success);

        let v = 0x12345678;
        let script = script! {
            { v }
            { CppInt32Gadget::from_positive_bitcoin_integer() }
            OP_PUSHBYTES_4 OP_OVER OP_PUSHNUM_6 OP_PUSHBYTES_52 OP_PUSHBYTES_18
            OP_EQUAL
        };
        let res = execute_script(script);
        assert!(res.success);
    }

    #[test]
    fn test_cpp_uint64_from_bitcoin_integer() {
        let v = 0x78345612;
        let script = script! {
            { v }
            { CppUInt64Gadget::from_bitcoin_integer() }
            OP_PUSHBYTES_8
                OP_PUSHBYTES_18 OP_PUSHNUM_6 OP_PUSHBYTES_52 OP_OVER
                OP_PUSHBYTES_0 OP_PUSHBYTES_0 OP_PUSHBYTES_0 OP_PUSHBYTES_0
            OP_EQUAL
        };
        let res = execute_script(script);
        assert!(res.success);
    }

    #[test]
    fn test_cpp_uint64_from_two_bitcoin_integers() {
        let vl = 0x78345612;
        let vh = 0x12345678;
        let script = script! {
            { vl }
            { vh }
            { CppUInt64Gadget::from_two_bitcoin_integers() }
            OP_PUSHBYTES_8
                OP_PUSHBYTES_18 OP_PUSHNUM_6 OP_PUSHBYTES_52 OP_OVER
                OP_OVER OP_PUSHNUM_6 OP_PUSHBYTES_52 OP_PUSHBYTES_18
            OP_EQUAL
        };
        let res = execute_script(script);
        assert!(res.success);
    }

    #[test]
    fn test_cpp_uint64_from_u64_in_16bit_limbs() {
        let mut script = script! {
            { U64::push_u64_le(&[100_000_000u64]) }
            { U64::push_u64_le(&[1234]) }
            { U64::push_u64_le(&[5678]) }

            { U64::toaltstack() }
            { U64::mul() }

            { U64::fromaltstack() }
            { U64::add(1, 0) }

            { CppUInt64Gadget::from_u64_in_16bit_limbs() }
        }
        .to_bytes();
        script.extend_from_slice(&[0x08, 0x2e, 0xa8, 0x36, 0xbb, 0x1c, 0x00, 0x00, 0x00]);
        script.extend_from_slice(
            script! {
                OP_EQUAL
            }
            .as_bytes(),
        );

        let res = execute_script(Script::from_bytes(script));
        assert!(res.success);
    }
}
