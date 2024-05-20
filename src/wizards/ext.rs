use crate::utils::pseudo::OP_CAT3;
use bitcoin::TapLeafHash;
use bitvm::treepp::*;

pub use crate::structures::tap_leaf_hash::TapLeafHashGadget as Step1TapLeafHashGadget;

pub use crate::structures::key_version::KeyVersionGadget as Step2KeyVersionGadget;

pub use crate::structures::codesep_pos::CodeSepPosGadget as Step3CodeSepPosGadget;

/// Gadget for the extension in taproot CheckSigVerify.
pub struct ExtGadget;

impl ExtGadget {
    /// Construct the extension using the tap leaf hash and the code separator position.
    pub fn from_constant(tap_leaf_hash: &TapLeafHash, code_sep_pos: Option<u32>) -> Script {
        script! {
            // tap leaf hash
            { Step1TapLeafHashGadget::from_constant(tap_leaf_hash) }
            // key version, currently always zero
            { Step2KeyVersionGadget::from_constant(0) }
            // code separator position
            // if no OP_CODESEPARATOR has ever been executed, the default 0xffffffff will be used.
            if code_sep_pos.is_some() {
                { Step3CodeSepPosGadget::from_constant(code_sep_pos.unwrap()) }
            } else {
                { Step3CodeSepPosGadget::no_code_sep_executed() }
            }
            OP_CAT3
        }
    }
}

#[cfg(test)]
mod test {
    use crate::wizards::ext::ExtGadget;
    use bitcoin::consensus::Encodable;
    use bitcoin::hashes::Hash;
    use bitcoin::TapLeafHash;
    use bitvm::treepp::*;
    use rand::{Rng, RngCore, SeedableRng};
    use rand_chacha::ChaCha20Rng;
    use sha2::Digest;

    #[test]
    fn test_ext() {
        let mut prng = ChaCha20Rng::seed_from_u64(0);

        for _ in 0..10 {
            let mut random_tap_data = [0u8; 40];
            prng.fill_bytes(&mut random_tap_data);

            let tap_leaf_hash = TapLeafHash::hash(&random_tap_data);
            let code_sep_pos = prng.gen::<u32>();

            let expected = {
                let mut bytes = vec![];

                tap_leaf_hash
                    .as_byte_array()
                    .consensus_encode(&mut bytes)
                    .unwrap();
                0u8.consensus_encode(&mut bytes).unwrap();
                code_sep_pos.consensus_encode(&mut bytes).unwrap();

                let mut sha256 = sha2::Sha256::new();
                Digest::update(&mut sha256, &bytes);

                sha256.finalize().to_vec()
            };

            let script = script! {
                { ExtGadget::from_constant(&tap_leaf_hash, Some(code_sep_pos)) }
                OP_SHA256

                { expected }
                OP_EQUAL
            };

            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }
}
