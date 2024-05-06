use crate::cpp_integer::CppUInt64Gadget;
use crate::script_pub_key::{ScriptPubKey, ScriptPubKeyGadget, ScriptPubKeyType};
use crate::variable_integer::VariableIntegerGadget;
use bitcoin::consensus::Encodable;
use bitcoin::opcodes::all::{
    OP_PUSHBYTES_1, OP_PUSHBYTES_3, OP_PUSHBYTES_5, OP_PUSHBYTES_8, OP_PUSHBYTES_9,
};
use bitcoin::Amount;
use bitvm::treepp::*;

pub struct TxOutGadget;

impl TxOutGadget {
    pub fn step_1_add_constant_value(value: Amount) -> Script {
        let v = value.to_sat();
        CppUInt64Gadget::from_constant(v)
    }

    pub fn step_1_add_provided_u64() -> Script {
        CppUInt64Gadget::from_u64_in_16bit_limbs()
    }

    pub fn step_2_add_constant_vi(len: usize) -> Script {
        let vi = bitcoin::VarInt::from(len as u64);

        let mut bytes = vec![];
        vi.consensus_encode(&mut bytes).unwrap();

        if vi.size() == 1 {
            Script::from_bytes(vec![OP_PUSHBYTES_1.to_u8(), bytes[0]])
        } else if vi.size() == 3 {
            Script::from_bytes(vec![OP_PUSHBYTES_3.to_u8(), bytes[0], bytes[1], bytes[2]])
        } else if vi.size() == 5 {
            Script::from_bytes(vec![
                OP_PUSHBYTES_5.to_u8(),
                bytes[0],
                bytes[1],
                bytes[2],
                bytes[3],
                bytes[4],
            ])
        } else if vi.size() == 9 {
            Script::from_bytes(vec![
                OP_PUSHBYTES_9.to_u8(),
                bytes[0],
                bytes[1],
                bytes[2],
                bytes[3],
                bytes[4],
                bytes[5],
                bytes[6],
                bytes[7],
                bytes[8],
            ])
        } else {
            unreachable!()
        }
    }

    pub fn step_2_add_provided_small_bitcoin_integer() -> Script {
        VariableIntegerGadget::encode_small_bitcoin_number()
    }

    pub fn step_3_add_constant_script_pub_key(script_pub_key: &ScriptPubKey) -> Script {
        ScriptPubKeyGadget::from_constant(script_pub_key)
    }

    pub fn step_3_add_provided_script_pub_key(script_pub_key_type: ScriptPubKeyType) -> Script {
        match script_pub_key_type {
            ScriptPubKeyType::P2WPKH => script! {
                OP_DUP OP_SIZE 20 OP_EQUALVERIFY
            },
            ScriptPubKeyType::P2WSH => script! {
                OP_DUP OP_SIZE 32 OP_EQUALVERIFY
            },
            ScriptPubKeyType::P2TR => script! {
                OP_DUP OP_SIZE 32 OP_EQUALVERIFY
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::script_pub_key::{script_pub_key_len, ScriptPubKey, ScriptPubKeyType};
    use crate::tx_out::TxOutGadget;
    use bitcoin::consensus::Encodable;
    use bitcoin::hashes::Hash;
    use bitcoin::key::TweakedPublicKey;
    use bitcoin::secp256k1::Secp256k1;
    use bitcoin::{Amount, ScriptBuf, TxOut, WPubkeyHash, WScriptHash, XOnlyPublicKey};
    use bitvm::treepp::*;
    use rand::{RngCore, SeedableRng};
    use rand_chacha::ChaCha20Rng;
    use sha2::digest::Update;
    use sha2::{Digest, Sha256};

    #[test]
    fn test_sha_prevouts() {
        let mut prng = ChaCha20Rng::seed_from_u64(0);

        for _ in 0..10 {
            let value_1 = Amount::from_sat(prng.next_u64());
            let value_2 = Amount::from_sat(prng.next_u64());
            let value_3 = Amount::from_sat(prng.next_u64());

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

                let tx_out_1 = TxOut {
                    value: value_1,
                    script_pubkey: ScriptBuf::new_p2wpkh(
                        &WPubkeyHash::from_slice(&pkhash).unwrap(),
                    ),
                };
                let tx_out_2 = TxOut {
                    value: value_2,
                    script_pubkey: ScriptBuf::new_p2wsh(
                        &WScriptHash::from_slice(&script_hash).unwrap(),
                    ),
                };
                let tx_out_3 = TxOut {
                    value: value_3,
                    script_pubkey: ScriptBuf::new_p2tr_tweaked(
                        TweakedPublicKey::dangerous_assume_tweaked(pubkey),
                    ),
                };

                tx_out_1.consensus_encode(&mut bytes).unwrap();
                tx_out_2.consensus_encode(&mut bytes).unwrap();
                tx_out_3.consensus_encode(&mut bytes).unwrap();

                let mut sha256 = Sha256::new();
                Update::update(&mut sha256, &bytes);

                let hash = sha256.finalize().to_vec();
                hash
            };

            let script = script! {
                { TxOutGadget::step_1_add_constant_value(value_1) }
                { TxOutGadget::step_2_add_constant_vi(script_pub_key_len(ScriptPubKeyType::P2WPKH)) }
                OP_CAT
                { TxOutGadget::step_3_add_constant_script_pub_key(&script_pub_key_1) }
                OP_CAT
                { TxOutGadget::step_1_add_constant_value(value_2) }
                OP_CAT
                { TxOutGadget::step_2_add_constant_vi(script_pub_key_len(ScriptPubKeyType::P2WSH)) }
                OP_CAT
                { TxOutGadget::step_3_add_constant_script_pub_key(&script_pub_key_2) }
                OP_CAT
                { TxOutGadget::step_1_add_constant_value(value_3) }
                OP_CAT
                { TxOutGadget::step_2_add_constant_vi(script_pub_key_len(ScriptPubKeyType::P2TR)) }
                OP_CAT
                { TxOutGadget::step_3_add_constant_script_pub_key(&script_pub_key_3) }
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
