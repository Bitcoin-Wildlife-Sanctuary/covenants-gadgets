use crate::treepp::*;
use sha2::Digest;

/// Enum for different hashtags used in tagged hashes.
#[derive(Clone, Eq, PartialEq)]
pub enum HashTag {
    /// tap leaf hash, which hashes a script
    TapLeaf,
    /// tap tweak hash, which is used for tweaking
    TapTweak,
    /// tap sig hash, which is to compute the sighash for Taproot signature verification
    TapSighash,
    /// BIP340 challenge, which is used in the Schnorr signature
    BIP340Challenge,
}

impl HashTag {
    /// Convert the hashtag to the corresponding preimage for SHA256.
    pub fn to_str(&self) -> &'static str {
        match self {
            HashTag::TapLeaf => "TapLeaf",
            HashTag::TapTweak => "TapTweak",
            HashTag::TapSighash => "TapSighash",
            HashTag::BIP340Challenge => "BIP0340/challenge",
        }
    }
}

/// Gadget for computing tagged hashes.
pub struct TaggedHashGadget;

impl TaggedHashGadget {
    /// Construct the tagged hash result from constant data.
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

    /// Compute the tagged hash from a constant tag and the provided message on the stack.
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

/// Obtain the hash of the tag that will be used to compute the tagged hash.
pub fn get_hashed_tag(tag: &'static str) -> Vec<u8> {
    let mut sha256 = sha2::Sha256::new();
    Digest::update(&mut sha256, tag.as_bytes());
    sha256.finalize().as_slice().to_vec()
}

#[cfg(test)]
mod test {
    use crate::structures::tagged_hash::{HashTag, TaggedHashGadget};
    use crate::treepp::*;
    use bitcoin::hashes::Hash;
    use bitcoin::TapSighash;
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
