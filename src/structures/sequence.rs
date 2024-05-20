use crate::internal_structures::cpp_int_32::CppInt32Gadget;
use crate::utils::push_u32_4bytes;
use bitcoin::Sequence;
use bitvm::treepp::*;

/// Gadget for the input sequence.
pub struct SequenceGadget;

impl SequenceGadget {
    /// Construct the input sequence from constant data.
    pub fn from_constant(sequence: &Sequence) -> Script {
        push_u32_4bytes(sequence.to_consensus_u32())
    }

    /// Construct the input sequence from a Bitcoin integer on the stack.
    pub fn from_bitcoin_integer() -> Script {
        CppInt32Gadget::from_bitcoin_integer()
    }

    /// Construct the input sequence from a positive Bitcoin integer on the stack.
    ///
    /// This is faster than `from_bitcoin_integer` because it doesn't handle negative numbers.
    pub fn from_positive_bitcoin_integer() -> Script {
        CppInt32Gadget::from_positive_bitcoin_integer()
    }
}

#[cfg(test)]
mod test {
    use crate::structures::sequence::SequenceGadget;
    use crate::utils::pseudo::OP_CAT3;
    use bitcoin::consensus::Encodable;
    use bitcoin::Sequence;
    use bitvm::treepp::*;
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;
    use sha2::digest::Update;
    use sha2::{Digest, Sha256};

    #[test]
    fn test_sha_sequences() {
        let mut prng = ChaCha20Rng::seed_from_u64(0);

        for _ in 0..100 {
            let seq_1 = Sequence::from_512_second_intervals(prng.gen::<u16>());
            let seq_2 = Sequence::from_512_second_intervals(prng.gen::<u16>());
            let seq_3 = Sequence::from_512_second_intervals(prng.gen::<u16>());

            let expected = {
                let mut bytes = vec![];
                seq_1.consensus_encode(&mut bytes).unwrap();
                seq_2.consensus_encode(&mut bytes).unwrap();
                seq_3.consensus_encode(&mut bytes).unwrap();

                let mut sha256 = Sha256::new();
                Update::update(&mut sha256, &bytes);

                let hash = sha256.finalize().to_vec();
                hash
            };

            let script = script! {
                { SequenceGadget::from_constant(&seq_1) }
                { SequenceGadget::from_constant(&seq_2) }
                { SequenceGadget::from_constant(&seq_3) }
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
