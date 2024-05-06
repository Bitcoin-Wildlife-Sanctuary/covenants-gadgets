use bitcoin::opcodes::all::OP_PUSHBYTES_8;
use bitvm::pseudo::OP_256MUL;
use bitvm::treepp::*;

pub type AmountGadget = CppUInt64Gadget;

pub struct CppUInt64Gadget;

impl CppUInt64Gadget {
    pub fn from_constant(v: u64) -> Script {
        Script::from_bytes(vec![
            OP_PUSHBYTES_8.to_u8(),
            (v & 0xff) as u8,
            ((v >> 8) & 0xff) as u8,
            ((v >> 16) & 0xff) as u8,
            ((v >> 24) & 0xff) as u8,
            ((v >> 32) & 0xff) as u8,
            ((v >> 40) & 0xff) as u8,
            ((v >> 48) & 0xff) as u8,
            ((v >> 56) & 0xff) as u8,
        ])
    }

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
    use crate::cpp_integer::{AmountGadget, CppUInt64Gadget};
    use crate::internal_structures::cpp_int_32::CppInt32Gadget;
    use bitcoin::consensus::Encodable;
    use bitcoin::Amount;
    use bitvm::bigint::U64;
    use bitvm::treepp::*;
    use rand::{RngCore, SeedableRng};
    use rand_chacha::ChaCha20Rng;
    use sha2::digest::Update;
    use sha2::{Digest, Sha256};

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

    #[test]
    fn test_sha_amounts() {
        let mut prng = ChaCha20Rng::seed_from_u64(0);

        for _ in 0..10 {
            let value_1 = Amount::from_sat(prng.next_u64());
            let value_2 = Amount::from_sat(prng.next_u64());
            let value_3 = Amount::from_sat(prng.next_u64());

            let expected = {
                let mut bytes = vec![];
                value_1.consensus_encode(&mut bytes).unwrap();
                value_2.consensus_encode(&mut bytes).unwrap();
                value_3.consensus_encode(&mut bytes).unwrap();

                let mut sha256 = Sha256::new();
                Update::update(&mut sha256, &bytes);

                let hash = sha256.finalize().to_vec();
                hash
            };

            let script = script! {
                { AmountGadget::from_constant(value_1.to_sat()) }
                { AmountGadget::from_constant(value_2.to_sat()) }
                OP_CAT
                { AmountGadget::from_constant(value_3.to_sat()) }
                OP_CAT
                OP_SHA256

                { expected }
                OP_EQUAL
            };

            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }
}
