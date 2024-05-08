use crate::utils::pseudo::OP_CAT2;
use bitcoin::TxOut;
use bitvm::treepp::*;

pub use crate::structures::amount::AmountGadget as Step1AmountGadget;
pub use crate::structures::script_pub_key::ScriptPubKeyGadget as Step2ScriptPubKeyGadget;

pub struct TxOutGadget;

impl TxOutGadget {
    pub fn from_constant(tx_out: &TxOut) -> Script {
        script! {
            { Step1AmountGadget::from_constant(tx_out.value) }
            { Step2ScriptPubKeyGadget::from_constant_scriptbuf(&tx_out.script_pubkey) }
            OP_CAT2
        }
    }
}

#[cfg(test)]
mod test {
    use crate::utils::pseudo::OP_CAT3;
    use crate::wizards::tx_out;
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
    fn test_sha_outputs() {
        let mut prng = ChaCha20Rng::seed_from_u64(0);

        for _ in 0..10 {
            let value_1 = Amount::from_sat(prng.next_u64());
            let value_2 = Amount::from_sat(prng.next_u64());
            let value_3 = Amount::from_sat(prng.next_u64());

            let mut pkhash = vec![0u8; 20];
            prng.fill_bytes(&mut pkhash);

            let mut script_hash = vec![0u8; 32];
            prng.fill_bytes(&mut script_hash);

            let secp = Secp256k1::new();
            let keypair = secp.generate_keypair(&mut prng);

            let pubkey = XOnlyPublicKey::from(keypair.1);

            let tx_out_1 = TxOut {
                value: value_1,
                script_pubkey: ScriptBuf::new_p2wpkh(&WPubkeyHash::from_slice(&pkhash).unwrap()),
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

            let expected = {
                let mut bytes = vec![];

                tx_out_1.consensus_encode(&mut bytes).unwrap();
                tx_out_2.consensus_encode(&mut bytes).unwrap();
                tx_out_3.consensus_encode(&mut bytes).unwrap();

                let mut sha256 = Sha256::new();
                Update::update(&mut sha256, &bytes);

                let hash = sha256.finalize().to_vec();
                hash
            };

            let script = script! {
                { tx_out::TxOutGadget::from_constant(&tx_out_1) }
                { tx_out::TxOutGadget::from_constant(&tx_out_2) }
                { tx_out::TxOutGadget::from_constant(&tx_out_3) }
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
