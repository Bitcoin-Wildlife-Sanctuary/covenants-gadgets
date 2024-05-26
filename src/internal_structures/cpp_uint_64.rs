use crate::internal_structures::cpp_int_32::CppInt32Gadget;
use crate::treepp::*;
use crate::utils::{push_u32_4bytes, push_u64_8bytes};
use bitvm::pseudo::OP_256MUL;

/// Gadget for 64-bit unsigned integer.
pub struct CppUInt64Gadget;

impl CppUInt64Gadget {
    /// Construct the 64-bit unsigned integer from constant data.
    pub fn from_constant(v: u64) -> Script {
        push_u64_8bytes(v)
    }

    /// Construct the 64-bit unsigned integer from a positive Bitcoin integer.
    pub fn from_positive_bitcoin_integer() -> Script {
        script! {
            // sanity check: input must be greater or equal to zero
            OP_DUP 0 OP_GREATERTHANOREQUAL OP_VERIFY

            // pad it to 8 bytes
            OP_SIZE 5 OP_LESSTHAN OP_IF { push_u32_4bytes(0) } OP_CAT OP_ENDIF
            OP_SIZE 7 OP_LESSTHAN OP_IF OP_PUSHBYTES_2 OP_PUSHBYTES_0 OP_PUSHBYTES_0 OP_CAT OP_ENDIF
            OP_SIZE 8 OP_LESSTHAN OP_IF OP_PUSHBYTES_1 OP_PUSHBYTES_0 OP_CAT OP_ENDIF
            OP_SIZE 8 OP_LESSTHAN OP_IF OP_PUSHBYTES_1 OP_PUSHBYTES_0 OP_CAT OP_ENDIF
        }
    }

    /// Construct the 64-bit unsigned integer from two Bitcoin integer, one representing
    /// the lower 4 bytes, one representing the higher 4 bytes.
    pub fn from_two_bitcoin_integers() -> Script {
        script! {
            { CppInt32Gadget::from_bitcoin_integer() }
            OP_SWAP
            { CppInt32Gadget::from_bitcoin_integer() }
            OP_SWAP OP_CAT
        }
    }

    /// Construct the 64-bit unsigned integer from u64 represented by four 16-bit limbs.
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
    use crate::internal_structures::cpp_uint_64::CppUInt64Gadget;
    use crate::treepp::*;
    use crate::utils::push_u64_8bytes;
    use bitvm::bigint::U64;

    #[test]
    fn test_cpp_uint64_from_bitcoin_integer() {
        let v = 0x78345612;
        let script = script! {
            { v }
            { CppUInt64Gadget::from_positive_bitcoin_integer() }
            { push_u64_8bytes(0x78345612) }
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
            { push_u64_8bytes(0x1234567878345612) }
            OP_EQUAL
        };
        let res = execute_script(script);
        assert!(res.success);
    }

    #[test]
    fn test_cpp_uint64_from_u64_in_16bit_limbs() {
        let script = script! {
            { U64::push_u64_le(&[100_000_000u64]) }
            { U64::push_u64_le(&[1234]) }
            { U64::push_u64_le(&[5678]) }

            { U64::toaltstack() }
            { U64::mul() }

            { U64::fromaltstack() }
            { U64::add(1, 0) }

            { CppUInt64Gadget::from_u64_in_16bit_limbs() }
            { push_u64_8bytes(100_000_000u64 * 1234 + 5678) }
            OP_EQUAL
        };

        let res = execute_script(script);
        assert!(res.success);
    }
}
