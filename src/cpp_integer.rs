use bitvm::pseudo::{OP_16MUL, OP_4MUL};
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

    pub fn from_u64_in_30bit_limbs() -> Script {
        script! {
            // stack:
            //   top 4 bits
            //   mid 30 bits
            //   lower 30 bits

            // remove the top 6 bits of the lower 30 bits
            for i in 0..6 {
                { 1 << (29 - i) }
                OP_2DUP OP_GREATERTHANOREQUAL OP_IF
                    OP_SUB 1
                OP_ELSE
                    OP_DROP 0
                OP_ENDIF
                OP_TOALTSTACK
            }

            // reshape it to 3 bytes
            OP_SIZE 3 OP_LESSTHAN OP_IF OP_PUSHBYTES_1 OP_PUSHBYTES_0 OP_CAT OP_ENDIF
            OP_SIZE 3 OP_LESSTHAN OP_IF OP_PUSHBYTES_1 OP_PUSHBYTES_0 OP_CAT OP_ENDIF
            OP_SIZE 3 OP_LESSTHAN OP_IF OP_PUSHBYTES_1 OP_PUSHBYTES_0 OP_CAT OP_ENDIF

            // move the third limb away
            OP_ROT

            OP_FROMALTSTACK OP_FROMALTSTACK
            OP_FROMALTSTACK OP_FROMALTSTACK
            OP_FROMALTSTACK OP_FROMALTSTACK

            OP_DUP OP_ADD OP_ADD
            OP_DUP OP_ADD OP_ADD
            OP_DUP OP_ADD OP_ADD
            OP_DUP OP_ADD OP_ADD
            OP_DUP OP_ADD OP_ADD

            // stack:
            //   lower 24 bits as 3 bytes
            //   top 4 bits
            //   mid 30 bits
            //   the top 6 bits combined

            OP_SWAP

            // remove the top 4 bits of the mid 30 bits
            { 1 << 29 }
            OP_2DUP OP_GREATERTHANOREQUAL OP_IF
                OP_SUB 1
            OP_ELSE
                OP_DROP 0
            OP_ENDIF
            OP_TOALTSTACK

            { 1 << 28 }
            OP_2DUP OP_GREATERTHANOREQUAL OP_IF
                OP_SUB 1
            OP_ELSE
                OP_DROP 0
            OP_ENDIF
            OP_TOALTSTACK

            { 1 << 27 }
            OP_2DUP OP_GREATERTHANOREQUAL OP_IF
                OP_SUB 1
            OP_ELSE
                OP_DROP 0
            OP_ENDIF
            OP_TOALTSTACK

            { 1 << 26 }
            OP_2DUP OP_GREATERTHANOREQUAL OP_IF
                OP_SUB 1
            OP_ELSE
                OP_DROP 0
            OP_ENDIF
            OP_TOALTSTACK

            OP_4MUL
            OP_ROT
            OP_DUP OP_ADD OP_ADD OP_ADD

            // reshape it to 4 bytes
            { CppInt32Gadget::from_positive_bitcoin_integer() }

            OP_ROT

            OP_FROMALTSTACK OP_FROMALTSTACK
            OP_FROMALTSTACK OP_FROMALTSTACK

            // stack:
            //   lower 28 bits as 4 bytes
            //   mid 26 bits + lower 2 bits as 4 bytes
            //   top 4 bits
            //   mid top bit
            //   mid top - 1 bit
            //   mid top - 2 bit
            //   mid top - 3 bit

            OP_DUP OP_ADD OP_ADD
            OP_DUP OP_ADD OP_ADD
            OP_DUP OP_ADD OP_ADD
            OP_SWAP OP_16MUL OP_ADD

            // reshape it to 4 bytes
            { CppInt32Gadget::from_positive_bitcoin_integer() }

            OP_ROT

            OP_CAT OP_CAT
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cpp_integer::{CppInt32Gadget, CppUInt64Gadget};
    use crate::U64;
    use bitcoin::opcodes::all::{OP_EQUAL, OP_PUSHBYTES_0, OP_PUSHBYTES_7};
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
    fn test_cpp_uint64_from_u64_in_30bit_limbs() {
        let mut script = script! {
            { U64::push_dec("1000000000000000000") }
            { U64::push_dec("1234") }
            { U64::push_dec("5678") }

            OP_ROT

            { U64::mul() }
            { U64::add(1, 0) }

            { CppUInt64Gadget::from_u64_in_30bit_limbs() }
            OP_RETURN
        }
        .to_bytes();
        /*script.extend_from_slice(&[0x08, 0x2e, 0x16, 0x08, 0xe0, 0xfc, 0xad, 0x30, 0xe5, 0x42]);
        script.extend_from_slice(script! {
            OP_EQUAL
        }.as_bytes());*/

        let res = execute_script(Script::from_bytes(script));
        println!("{:8}", res.final_stack);
        assert!(res.success);
    }
}
