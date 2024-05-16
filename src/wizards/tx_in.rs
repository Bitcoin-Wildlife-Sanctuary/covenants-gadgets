use crate::utils::pseudo::OP_CAT3;
use bitcoin::TxIn;
use bitvm::treepp::*;

pub use crate::wizards::outpoint as step1_outpoint;
pub use crate::wizards::outpoint::OutPointGadget as Step1OutPointGadget;

pub use crate::structures::script_sig::ScriptSigGadget as Step2ScriptSigGadget;

pub use crate::structures::sequence::SequenceGadget as Step3SequenceGadget;

pub struct TxInGadget;

impl TxInGadget {
    pub fn from_constant(tx_in: &TxIn) -> Script {
        assert!(tx_in.script_sig.is_empty());

        script! {
            { Step1OutPointGadget::from_constant(&tx_in.previous_output) }
            { Step2ScriptSigGadget::segregated_witness() }
            { Step3SequenceGadget::from_constant(&tx_in.sequence) }
            OP_CAT3
        }
    }
}

#[cfg(test)]
mod test {
    use crate::wizards::tx_in::TxInGadget;
    use bitcoin::consensus::Encodable;
    use bitcoin::hashes::Hash;
    use bitcoin::{OutPoint, ScriptBuf, Sequence, TxIn, Txid, Witness};
    use bitvm::treepp::*;
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;
    use sha2::Digest;

    #[test]
    fn test_tx_in() {
        let mut prng = ChaCha20Rng::seed_from_u64(0);

        for _ in 0..10 {
            let outpoint = {
                let mut engine = bitcoin::hashes::sha256::Hash::engine();
                let r = prng.gen::<u32>();
                r.consensus_encode(&mut engine).unwrap();
                OutPoint::new(Txid::from_engine(engine), prng.gen::<u32>())
            };

            let seq = Sequence::from_512_second_intervals(prng.gen::<u16>());

            let tx_in = TxIn {
                previous_output: outpoint,
                script_sig: ScriptBuf::new(),
                sequence: seq,
                witness: Witness::new(),
            };

            let expected = {
                let mut bytes = vec![];
                tx_in.consensus_encode(&mut bytes).unwrap();

                let mut sha256 = sha2::Sha256::new();
                Digest::update(&mut sha256, &bytes);

                sha256.finalize().to_vec()
            };

            let script = script! {
                { TxInGadget::from_constant(&tx_in) }
                OP_SHA256
                { expected }
                OP_EQUAL
            };

            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }
}
