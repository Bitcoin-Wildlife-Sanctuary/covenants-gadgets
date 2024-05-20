use crate::internal_structures::variable_length_integer::VariableLengthIntegerGadget;
use bitcoin::opcodes::all::{
    OP_PUSHBYTES_20, OP_PUSHBYTES_22, OP_PUSHBYTES_32, OP_PUSHBYTES_34, OP_PUSHNUM_1,
};
use bitcoin::opcodes::OP_0;
use bitcoin::ScriptBuf;
use bitvm::treepp::*;

/// Wrapper for supported script pub keys.
pub enum ScriptPubKey {
    /// pay-to-witness-public-key-hash, given the 20-byte public key hash
    P2WPKH(Vec<u8>),
    /// pay-to-witness-script-hash, given the 32-byte script hash
    P2WSH(Vec<u8>),
    /// pay-to-taproot, given the 32-byte taproot point
    P2TR(Vec<u8>),
}

/// Enums for different types of supported script pub keys.
#[derive(Clone, Copy)]
pub enum ScriptPubKeyType {
    /// pay-to-witness-public-key-hash
    P2WPKH,
    /// pay-to-witness-script-hash
    P2WSH,
    /// pay-to-taproot
    P2TR,
}

/// Gadget for the script public key.
pub struct ScriptPubKeyGadget;

impl ScriptPubKeyGadget {
    /// Construct the script public key from the witness public key hash.
    pub fn p2wpkh_from_constant_hash(wpkhash: &[u8]) -> Script {
        assert_eq!(wpkhash.len(), 20);

        let mut script = vec![
            OP_PUSHBYTES_22.to_u8(),
            OP_0.to_u8(),
            OP_PUSHBYTES_20.to_u8(),
        ];
        script.extend_from_slice(wpkhash);
        Script::from_bytes(script)
    }

    /// Construct the script public key from the witness script hash.
    pub fn p2wsh_from_constant_hash(wsh: &[u8]) -> Script {
        assert_eq!(wsh.len(), 32);

        let mut script = vec![
            OP_PUSHBYTES_34.to_u8(),
            OP_0.to_u8(),
            OP_PUSHBYTES_32.to_u8(),
        ];
        script.extend_from_slice(wsh);
        Script::from_bytes(script)
    }

    /// Construct the script public key from the taproot point.
    pub fn p2tr_from_public_key(public_key: &[u8]) -> Script {
        assert_eq!(public_key.len(), 32);

        let mut script = vec![
            OP_PUSHBYTES_34.to_u8(),
            OP_PUSHNUM_1.to_u8(),
            OP_PUSHBYTES_32.to_u8(),
        ];
        script.extend_from_slice(public_key);
        Script::from_bytes(script)
    }

    /// Construct the script public key from the `ScriptPubKey` struct.
    pub fn from_constructor(script_pub_key: &ScriptPubKey) -> Script {
        match script_pub_key {
            ScriptPubKey::P2WPKH(pkhash) => script! {
                { VariableLengthIntegerGadget::from_constant(22) }
                { ScriptPubKeyGadget::p2wpkh_from_constant_hash(pkhash) }
                OP_CAT
            },
            ScriptPubKey::P2WSH(script_hash) => script! {
                { VariableLengthIntegerGadget::from_constant(34) }
                { ScriptPubKeyGadget::p2wsh_from_constant_hash(script_hash) }
                OP_CAT
            },
            ScriptPubKey::P2TR(public_key) => script! {
                { VariableLengthIntegerGadget::from_constant(34) }
                { ScriptPubKeyGadget::p2tr_from_public_key(public_key) }
                OP_CAT
            },
        }
    }

    /// Construct the script public key from constant data.
    pub fn from_constant(script_buf: &ScriptBuf) -> Script {
        script! {
            { VariableLengthIntegerGadget::from_constant(script_buf.len()) }
            { script_buf.to_bytes() }
            OP_CAT
        }
    }

    /// Construct the script public key from the provided data on the stack.
    ///
    /// It checks if the provided data has the right number of bytes.
    pub fn from_provided(script_pub_key_type: ScriptPubKeyType) -> Script {
        match script_pub_key_type {
            ScriptPubKeyType::P2WPKH => script! {
                OP_DUP OP_SIZE 20 OP_EQUALVERIFY
                OP_PUSHBYTES_1 OP_PUSHBYTES_22 OP_SWAP OP_CAT
            },
            ScriptPubKeyType::P2WSH => script! {
                OP_DUP OP_SIZE 32 OP_EQUALVERIFY
                OP_PUSHBYTES_1 OP_PUSHBYTES_34 OP_SWAP OP_CAT
            },
            ScriptPubKeyType::P2TR => script! {
                OP_DUP OP_SIZE 32 OP_EQUALVERIFY
                OP_PUSHBYTES_1 OP_PUSHBYTES_34 OP_SWAP OP_CAT
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::structures::script_pub_key::{ScriptPubKey, ScriptPubKeyGadget};
    use crate::utils::pseudo::OP_CAT3;
    use bitcoin::consensus::Encodable;
    use bitcoin::hashes::Hash;
    use bitcoin::key::TweakedPublicKey;
    use bitcoin::secp256k1::Secp256k1;
    use bitcoin::{ScriptBuf, WPubkeyHash, WScriptHash, XOnlyPublicKey};
    use bitvm::treepp::*;
    use rand::{RngCore, SeedableRng};
    use rand_chacha::ChaCha20Rng;
    use sha2::digest::Update;
    use sha2::{Digest, Sha256};

    #[test]
    fn test_sha_scriptpubkeys() {
        let mut prng = ChaCha20Rng::seed_from_u64(0);

        for _ in 0..10 {
            let mut pkhash = vec![0u8; 20];
            prng.fill_bytes(&mut pkhash);
            let script_pub_key_1 = ScriptPubKey::P2WPKH(pkhash.clone());

            let mut script_hash = vec![0u8; 32];
            prng.fill_bytes(&mut script_hash);
            let script_pub_key_2 = ScriptPubKey::P2WSH(script_hash.clone());

            let secp = Secp256k1::new();
            let keypair = secp.generate_keypair(&mut prng);

            let pubkey = XOnlyPublicKey::from(keypair.1);
            let script_pub_key_3 = ScriptPubKey::P2TR(pubkey.serialize().to_vec());

            let expected = {
                let mut bytes = vec![];

                let script_pubkey_1 =
                    ScriptBuf::new_p2wpkh(&WPubkeyHash::from_slice(&pkhash).unwrap());
                let script_pubkey_2 =
                    ScriptBuf::new_p2wsh(&WScriptHash::from_slice(&script_hash).unwrap());
                let script_pubkey_3 =
                    ScriptBuf::new_p2tr_tweaked(TweakedPublicKey::dangerous_assume_tweaked(pubkey));

                script_pubkey_1.consensus_encode(&mut bytes).unwrap();
                script_pubkey_2.consensus_encode(&mut bytes).unwrap();
                script_pubkey_3.consensus_encode(&mut bytes).unwrap();

                let mut sha256 = Sha256::new();
                Update::update(&mut sha256, &bytes);

                let hash = sha256.finalize().to_vec();
                hash
            };

            let script = script! {
                { ScriptPubKeyGadget::from_constructor(&script_pub_key_1) }
                { ScriptPubKeyGadget::from_constructor(&script_pub_key_2) }
                { ScriptPubKeyGadget::from_constructor(&script_pub_key_3) }
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
