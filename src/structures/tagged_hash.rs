use bitvm::treepp::*;
use sha2::Digest;

#[derive(Clone, Eq, PartialEq)]
pub enum HashTag {
    TapLeaf,
    TapTweak,
    TapSighash,
}

impl HashTag {
    pub fn to_str(&self) -> &'static str {
        match self {
            HashTag::TapLeaf => "TapLeaf",
            HashTag::TapTweak => "TapTweak",
            HashTag::TapSighash => "TapSighash",
        }
    }
}

pub struct TaggedHashGadget;

impl TaggedHashGadget {
    pub fn from_constant(hashtag: &HashTag, msg: &[u8]) -> Script {
        let hashed_tag = get_hashed_tag(hashtag.to_str());

        let mut sha256 = sha2::Sha256::new();
        Digest::update(&mut sha256, &hashed_tag);
        Digest::update(&mut sha256, &hashed_tag);
        Digest::update(&mut sha256, msg);

        let bytes = sha256.finalize().as_slice().to_vec();
        script! {
            { bytes }
        }
    }

    pub fn from_provided(hashtag: &HashTag) -> Script {
        let hashed_tag = get_hashed_tag(hashtag.to_str());
        script! {
            { hashed_tag }
            OP_DUP OP_CAT
            OP_SWAP OP_CAT
            OP_SHA256
        }
    }
}

pub fn get_hashed_tag(tag: &'static str) -> Vec<u8> {
    let mut sha256 = sha2::Sha256::new();
    Digest::update(&mut sha256, tag.as_bytes());
    sha256.finalize().as_slice().to_vec()
}

#[cfg(test)]
mod test {
    use crate::structures::tagged_hash::{HashTag, TaggedHashGadget};
    use bitcoin::hashes::Hash;
    use bitcoin::TapSighash;
    use bitvm::treepp::*;
    use rand::{RngCore, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    #[test]
    fn test_tagged_hash() {
        let mut prng = ChaCha20Rng::seed_from_u64(0);

        let mut bytes = vec![0u8; 40];
        prng.fill_bytes(&mut bytes);

        let expected = {
            let tap_sig_hash = TapSighash::hash(&bytes);

            let mut bytes = vec![0u8; 32];
            bytes.copy_from_slice(tap_sig_hash.as_byte_array());

            bytes
        };

        let script = script! {
            { TaggedHashGadget::from_constant(&HashTag::TapSighash, &bytes) }
            { expected.clone() }
            OP_EQUAL
        };

        let exec_result = execute_script(script);
        assert!(exec_result.success);

        let script = script! {
            { bytes }
            { TaggedHashGadget::from_provided(&HashTag::TapSighash) }
            { expected }
            OP_EQUAL
        };

        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }
}
