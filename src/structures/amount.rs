use crate::internal_structures::cpp_uint_64::CppUInt64Gadget;
use bitcoin::Amount;
use bitvm::treepp::*;

pub struct AmountGadget;

impl AmountGadget {
    pub fn from_constant(amount: Amount) -> Script {
        CppUInt64Gadget::from_constant(amount.to_sat())
    }

    pub fn from_bitcoin_integer() -> Script {
        CppUInt64Gadget::from_bitcoin_integer()
    }

    pub fn from_two_bitcoin_integers() -> Script {
        CppUInt64Gadget::from_two_bitcoin_integers()
    }

    pub fn from_u64_in_16bit_limbs() -> Script {
        CppUInt64Gadget::from_u64_in_16bit_limbs()
    }
}

#[cfg(test)]
mod test {
    use crate::structures::amount::AmountGadget;
    use crate::utils::pseudo::OP_CAT3;
    use bitcoin::consensus::Encodable;
    use bitcoin::Amount;
    use bitvm::treepp::*;
    use rand::{RngCore, SeedableRng};
    use rand_chacha::ChaCha20Rng;
    use sha2::digest::Update;
    use sha2::{Digest, Sha256};

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
                { AmountGadget::from_constant(value_1) }
                { AmountGadget::from_constant(value_2) }
                { AmountGadget::from_constant(value_3) }
                OP_CAT3
                OP_SHA256

                { expected }
                OP_EQUAL
            };

            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }
}
