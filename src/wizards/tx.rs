use bitcoin::Transaction;
use bitvm::treepp::*;

pub use crate::internal_structures::variable_length_integer::VariableLengthIntegerGadget as Step2InCounterGadget;
pub use crate::internal_structures::variable_length_integer::VariableLengthIntegerGadget as Step4OutCounterGadget;
pub use crate::structures::locktime::LockTimeGadget as Step6LockTimeGadget;
pub use crate::structures::version::VersionGadget as Step1VersionGadget;
use crate::utils::pseudo::OP_CAT4;
pub use crate::wizards::tx_in as step3_input;
pub use crate::wizards::tx_in::TxInGadget as Step3InputGadget;
pub use crate::wizards::tx_out as step5_output;
pub use crate::wizards::tx_out::TxOutGadget as Step5OutputGadget;

pub struct TxGadget;

impl TxGadget {
    pub fn from_constant(tx: &Transaction) -> Script {
        script! {
            { Step1VersionGadget::from_constant(tx.version) }
            { Step2InCounterGadget::from_constant(tx.input.len()) }
            for entry in tx.input.iter() {
                { Step3InputGadget::from_constant(entry) }
                OP_CAT
            }
            { Step4OutCounterGadget::from_constant(tx.output.len()) }
            for entry in tx.output.iter() {
                { Step5OutputGadget::from_constant(entry) }
                OP_CAT
            }
            { Step6LockTimeGadget::from_constant_absolute(tx.lock_time) }
            OP_CAT4
        }
    }

    pub fn hash() -> Script {
        script! {
            OP_SHA256 OP_SHA256
        }
    }
}

#[cfg(test)]
mod test {
    use crate::utils::pseudo::{OP_CAT2, OP_CAT3};
    use crate::wizards::tx;
    use bitcoin::consensus::{Decodable, Encodable};
    use bitcoin::Transaction;
    use bitvm::treepp::*;

    #[test]
    fn test_tx() {
        // Murchandamus suggested this in the Bitcoin stackexchange
        // 2eb8dbaa346d4be4e82fe444c2f0be00654d8cfd8c4a9a61b11aeaab8c00b272

        let hex = "010000000001022373cf02ce7df6500ae46a4a0fbbb1b636d2debed8f2df91e2415627397a34090000000000fdffffff88c23d928893cd3509845516cf8411b7cab2738c054cc5ce7e4bde9586997c770000000000fdffffff0200000000000000002b6a29676d20746170726f6f7420f09fa5952068747470733a2f2f626974636f696e6465766b69742e6f72676e9e1100000000001976a91405070d0290da457409a37db2e294c1ffbc52738088ac04410adf90fd381d4a13c3e73740b337b230701189ed94abcb4030781635f035e6d3b50b8506470a68292a2bc74745b7a5732a28254b5f766f09e495929ec308090b01004620c13e6d193f5d04506723bd67abcc5d31b610395c445ac6744cb0a1846b3aabaeac20b0e2e48ad7c3d776cf6f2395c504dc19551268ea7429496726c5d5bf72f9333cba519c21c0000000000000000000000000000000000000000000000000000000000000000104414636070d21adc8280735383102f7a0f5978cea257777a23934dd3b458b79bf388aca218e39e23533a059da173e402c4fc5e3375e1f839efb22e9a5c2a815b07301004620c13e6d193f5d04506723bd67abcc5d31b610395c445ac6744cb0a1846b3aabaeac20b0e2e48ad7c3d776cf6f2395c504dc19551268ea7429496726c5d5bf72f9333cba519c21c0000000000000000000000000000000000000000000000000000000000000000100000000";
        let bytes = hex::decode(hex).unwrap();
        let tx = Transaction::consensus_decode(&mut bytes.as_slice()).unwrap();

        let mut tx_preimage = vec![];
        tx.version.consensus_encode(&mut tx_preimage).unwrap();
        tx.input.consensus_encode(&mut tx_preimage).unwrap();
        tx.output.consensus_encode(&mut tx_preimage).unwrap();
        tx.lock_time.consensus_encode(&mut tx_preimage).unwrap();

        let txid = tx.compute_txid();

        let script = script! {
            { tx::Step1VersionGadget::from_constant(tx.version) }
            { tx::Step2InCounterGadget::from_constant(tx.input.len()) }
            OP_CAT2

            for input in tx.input.iter() {
                { tx::step3_input::step1_outpoint::Step1TxIdGadget::from_constant(input.previous_output.txid) }
                { tx::step3_input::step1_outpoint::Step2IndexGadget::from_constant(input.previous_output.vout) }
                OP_CAT2
                { tx::step3_input::Step2ScriptSigGadget::segregated_witness() }
                { tx::step3_input::Step3SequenceGadget::from_constant(input.sequence) }
                OP_CAT3

                OP_CAT2
            }

            { tx::Step4OutCounterGadget::from_constant(tx.output.len()) }
            OP_CAT2

            for output in tx.output.iter() {
                { tx::step5_output::Step1AmountGadget::from_constant(output.value) }
                { tx::step5_output::Step2ScriptPubKeyGadget::from_constant_scriptbuf(&output.script_pubkey) }
                OP_CAT2

                OP_CAT2
            }

            { tx::Step6LockTimeGadget::from_constant_absolute(tx.lock_time) }
            OP_CAT2

            { tx_preimage }
            OP_DUP OP_TOALTSTACK
            OP_EQUALVERIFY

            OP_FROMALTSTACK
            OP_SHA256 OP_SHA256
            { AsRef::<[u8]>::as_ref(&txid).to_vec() }
            OP_EQUAL
        };

        let exec_script = execute_script(script);
        assert!(exec_script.success);
    }
}
