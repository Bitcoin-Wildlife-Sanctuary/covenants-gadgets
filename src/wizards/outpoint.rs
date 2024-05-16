use crate::utils::pseudo::OP_CAT2;
use bitcoin::OutPoint;
use bitvm::treepp::*;

pub use crate::structures::txid::TxIdGadget as Step1TxIdGadget;

pub use crate::internal_structures::cpp_int_32::CppInt32Gadget as Step2IndexGadget;

pub struct OutPointGadget;

impl OutPointGadget {
    pub fn from_constant(outpoint: &OutPoint) -> Script {
        script! {
            { Step1TxIdGadget::from_constant(outpoint.txid) }
            { Step2IndexGadget::from_constant(outpoint.vout) }
            OP_CAT2
        }
    }
}

#[cfg(test)]
mod test {
    use crate::utils::pseudo::OP_CAT3;
    use crate::wizards::outpoint::OutPointGadget;
    use bitcoin::consensus::Encodable;
    use bitcoin::hashes::Hash;
    use bitcoin::{OutPoint, Txid};
    use bitvm::treepp::*;
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;
    use sha2::digest::Update;
    use sha2::{Digest, Sha256};

    #[test]
    fn test_sha_prevouts() {
        let mut prng = ChaCha20Rng::seed_from_u64(0);

        for _ in 0..10 {
            let outpoint_1 = {
                let mut engine = bitcoin::hashes::sha256::Hash::engine();
                let r = prng.gen::<u32>();
                r.consensus_encode(&mut engine).unwrap();
                OutPoint::new(Txid::from_engine(engine), prng.gen::<u32>())
            };

            let outpoint_2 = {
                let mut engine = bitcoin::hashes::sha256::Hash::engine();
                let r = prng.gen::<u32>();
                r.consensus_encode(&mut engine).unwrap();
                OutPoint::new(Txid::from_engine(engine), prng.gen::<u32>())
            };

            let outpoint_3 = {
                let mut engine = bitcoin::hashes::sha256::Hash::engine();
                let r = prng.gen::<u32>();
                r.consensus_encode(&mut engine).unwrap();
                OutPoint::new(Txid::from_engine(engine), prng.gen::<u32>())
            };

            let expected = {
                let mut bytes = vec![];
                outpoint_1.consensus_encode(&mut bytes).unwrap();
                outpoint_2.consensus_encode(&mut bytes).unwrap();
                outpoint_3.consensus_encode(&mut bytes).unwrap();

                let mut sha256 = Sha256::new();
                Update::update(&mut sha256, &bytes);

                let hash = sha256.finalize().to_vec();
                hash
            };

            let script = script! {
                { OutPointGadget::from_constant(&outpoint_1) }
                { OutPointGadget::from_constant(&outpoint_2) }
                { OutPointGadget::from_constant(&outpoint_3) }
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
