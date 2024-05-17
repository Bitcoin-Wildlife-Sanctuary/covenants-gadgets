pub mod tx_data_part1 {
    use crate::utils::pseudo::{OP_CAT2, OP_CAT4};
    use bitcoin::{Amount, OutPoint, ScriptBuf, Sequence};
    use bitvm::treepp::*;

    pub use crate::wizards::outpoint as step1_outpoint;
    pub use crate::wizards::outpoint::OutPointGadget as Step1OutPointGadget;

    pub use crate::structures::amount::AmountGadget as Step2AmountGadget;

    pub use crate::structures::script_pub_key::ScriptPubKeyGadget as Step3ScriptPubKeyGadget;

    pub use crate::structures::sequence::SequenceGadget as Step4SequenceGadget;

    pub struct TxDataPart1Gadget;
    impl TxDataPart1Gadget {
        pub fn from_constant(
            outpoints: &[OutPoint],
            amounts: &[Amount],
            script_pub_keys: &[ScriptBuf],
            sequences: &[Sequence],
        ) -> Script {
            script! {
                OP_PUSHBYTES_0

                for outpoint in outpoints.iter() {
                    { Step1OutPointGadget::from_constant(outpoint) }
                    OP_CAT2
                }

                OP_SHA256

                OP_PUSHBYTES_0

                for amount in amounts.iter() {
                    { Step2AmountGadget::from_constant(amount) }
                    OP_CAT2
                }
                OP_SHA256

                OP_PUSHBYTES_0

                for script_pub_key in script_pub_keys.iter() {
                    { Step3ScriptPubKeyGadget::from_constant(script_pub_key)}
                    OP_CAT2
                }
                OP_SHA256

                OP_PUSHBYTES_0

                for sequence in sequences.iter() {
                    { Step4SequenceGadget::from_constant(sequence) }
                    OP_CAT2
                }
                OP_SHA256

                OP_CAT4
            }
        }
    }
}

pub mod tx_data_part2 {
    use crate::utils::pseudo::OP_CAT2;
    use bitcoin::TxOut;
    use bitvm::treepp::*;

    pub use crate::wizards::tx_out as step1_tx_out;
    pub use crate::wizards::tx_out::TxOutGadget as Step1TxOutGadget;

    pub struct TxDataPart2Gadget;

    impl TxDataPart2Gadget {
        pub fn from_constant(outputs: &[TxOut]) -> Script {
            script! {
                OP_PUSHBYTES_0

                for output in outputs.iter() {
                    { Step1TxOutGadget::from_constant(output) }
                    OP_CAT2
                }
                OP_SHA256
            }
        }
    }
}

pub mod data_input_if_anyonecanpay {
    use bitcoin::{Amount, OutPoint, ScriptBuf, Sequence};
    use bitvm::treepp::*;

    pub use crate::wizards::outpoint as step1_outpoint;
    pub use crate::wizards::outpoint::OutPointGadget as Step1OutPointGadget;

    pub use crate::structures::amount::AmountGadget as Step2AmountGadget;

    pub use crate::structures::script_pub_key::ScriptPubKeyGadget as Step3ScriptPubKeyGadget;

    pub use crate::structures::sequence::SequenceGadget as Step4SequenceGadget;
    use crate::utils::pseudo::OP_CAT4;

    pub struct DataInputPart1Gadget;

    impl DataInputPart1Gadget {
        pub fn from_constant(
            outpoint: &OutPoint,
            amount: &Amount,
            script_pub_key: &ScriptBuf,
            sequence: &Sequence,
        ) -> Script {
            script! {
                { Step1OutPointGadget::from_constant(outpoint) }
                { Step2AmountGadget::from_constant(amount) }
                { Step3ScriptPubKeyGadget::from_constant(script_pub_key) }
                { Step4SequenceGadget::from_constant(sequence) }

                OP_CAT4
            }
        }
    }
}

use crate::utils::pseudo::{OP_CAT2, OP_CAT4};
use bitcoin::{Amount, OutPoint, ScriptBuf, Sequence, TapLeafHash, TapSighashType, Transaction};
use bitvm::treepp::*;

pub use crate::structures::epoch::EpochGadget as Step1EpochGadget;

pub use crate::structures::hashtype::HashTypeGadget as Step2HashTypeGadget;

pub use crate::structures::version::VersionGadget as Step3VersionGadget;

pub use crate::structures::locktime::LockTimeGadget as Step4LockTimeGadget;

pub use tx_data_part1 as step5_tx_data_part1_if_not_anyonecanpay;
pub use tx_data_part1::TxDataPart1Gadget as Step5TxPart1GadgetIfNotAnyOneCanPay;

pub use tx_data_part2 as step6_tx_data_part2_if_not_none_or_single;
pub use tx_data_part2::TxDataPart2Gadget as Step6TxPart2GadgetIfNotNoneOrSingle;

pub use crate::structures::spend_type::SpendTypeGadget as Step7SpendTypeGadget;

pub use data_input_if_anyonecanpay as step8_data_input_part_if_anyonecanpay;
pub use data_input_if_anyonecanpay::DataInputPart1Gadget as Step8DataInputPart1GadgetIfAnyOneCanPay;

pub use crate::internal_structures::cpp_int_32::CppInt32Gadget as Step9InputIndexGadgetIfNotAnyOneCanPay;

pub use crate::structures::annex::AnnexGadget as Step10AnnexGadgetIfPresent;

pub use crate::wizards::tx_out as step11_this_output_if_single;
pub use crate::wizards::tx_out::TxOutGadget as Step11ThisOutputGadgetIfSingle;

pub use crate::wizards::ext as step12_ext;
pub use crate::wizards::ext::ExtGadget as Step12ExtGadget;

pub struct TapCSVPreImageGadget;

impl TapCSVPreImageGadget {
    pub fn from_constant(
        tx: &Transaction,
        input_amounts: &[Amount],
        input_script_pub_keys: &[ScriptBuf],
        this_input_idx: usize,
        tap_leaf_hash: &TapLeafHash,
        code_sep_pos: Option<u32>,
        hash_type: &TapSighashType,
    ) -> Script {
        assert_eq!(tx.input.len(), input_amounts.len());

        script! {
            { Step1EpochGadget::default() }
            { Step2HashTypeGadget::from_constant(hash_type) }
            { Step3VersionGadget::from_constant(&tx.version) }
            { Step4LockTimeGadget::from_constant_absolute(&tx.lock_time) }
            OP_CAT4
            if ![TapSighashType::AllPlusAnyoneCanPay, TapSighashType::NonePlusAnyoneCanPay, TapSighashType::SinglePlusAnyoneCanPay].contains(hash_type) {
                { Step5TxPart1GadgetIfNotAnyOneCanPay::from_constant(
                    &tx.input.iter().map(|x| x.previous_output.clone()).collect::<Vec<OutPoint>>(),
                    input_amounts,
                    input_script_pub_keys,
                    &tx.input.iter().map(|x| x.sequence.clone()).collect::<Vec<Sequence>>(),
                ) }
                OP_CAT2
            }
            if [TapSighashType::All, TapSighashType::Default, TapSighashType::AllPlusAnyoneCanPay].contains(hash_type) {
                { Step6TxPart2GadgetIfNotNoneOrSingle::from_constant(&tx.output) }
                OP_CAT2
            }
            { Step7SpendTypeGadget::from_constant(1, false) }
            if [TapSighashType::AllPlusAnyoneCanPay, TapSighashType::NonePlusAnyoneCanPay, TapSighashType::SinglePlusAnyoneCanPay].contains(hash_type) {
                { Step8DataInputPart1GadgetIfAnyOneCanPay::from_constant(
                    &tx.input[this_input_idx].previous_output,
                    &input_amounts[this_input_idx],
                    &input_script_pub_keys[this_input_idx],
                    &tx.input[this_input_idx].sequence,
                ) }
            } else {
                { Step9InputIndexGadgetIfNotAnyOneCanPay::from_constant(this_input_idx as u32) }
            }
            { Step10AnnexGadgetIfPresent::none() }
            OP_CAT4
            if [TapSighashType::Single, TapSighashType::SinglePlusAnyoneCanPay].contains(hash_type) {
                { Step11ThisOutputGadgetIfSingle::from_constant(&tx.output[this_input_idx]) }
                OP_CAT2
            }
            { Step12ExtGadget::from_constant(tap_leaf_hash, code_sep_pos) }
            OP_CAT2
        }
    }
}

#[cfg(test)]
mod test {
    use crate::utils::pseudo::{OP_CAT2, OP_CAT3, OP_CAT4};
    use crate::wizards::tap_csv_preimage::{
        step11_this_output_if_single, step12_ext, step5_tx_data_part1_if_not_anyonecanpay,
        step8_data_input_part_if_anyonecanpay, Step10AnnexGadgetIfPresent, Step1EpochGadget,
        Step2HashTypeGadget, Step3VersionGadget, Step4LockTimeGadget,
        Step6TxPart2GadgetIfNotNoneOrSingle, Step7SpendTypeGadget,
        Step9InputIndexGadgetIfNotAnyOneCanPay, TapCSVPreImageGadget,
    };
    use bitcoin::consensus::Decodable;
    use bitcoin::hashes::Hash;
    use bitcoin::sighash::{Prevouts, SighashCache};
    use bitcoin::{Amount, ScriptBuf, TapLeafHash, TapSighashType, Transaction, TxOut};
    use bitvm::treepp::*;
    use rand::{RngCore, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    #[test]
    fn test_tap_csv_preimage() {
        // Murchandamus suggested this in the Bitcoin stackexchange
        // 2eb8dbaa346d4be4e82fe444c2f0be00654d8cfd8c4a9a61b11aeaab8c00b272

        let hex = "010000000001022373cf02ce7df6500ae46a4a0fbbb1b636d2debed8f2df91e2415627397a34090000000000fdffffff88c23d928893cd3509845516cf8411b7cab2738c054cc5ce7e4bde9586997c770000000000fdffffff0200000000000000002b6a29676d20746170726f6f7420f09fa5952068747470733a2f2f626974636f696e6465766b69742e6f72676e9e1100000000001976a91405070d0290da457409a37db2e294c1ffbc52738088ac04410adf90fd381d4a13c3e73740b337b230701189ed94abcb4030781635f035e6d3b50b8506470a68292a2bc74745b7a5732a28254b5f766f09e495929ec308090b01004620c13e6d193f5d04506723bd67abcc5d31b610395c445ac6744cb0a1846b3aabaeac20b0e2e48ad7c3d776cf6f2395c504dc19551268ea7429496726c5d5bf72f9333cba519c21c0000000000000000000000000000000000000000000000000000000000000000104414636070d21adc8280735383102f7a0f5978cea257777a23934dd3b458b79bf388aca218e39e23533a059da173e402c4fc5e3375e1f839efb22e9a5c2a815b07301004620c13e6d193f5d04506723bd67abcc5d31b610395c445ac6744cb0a1846b3aabaeac20b0e2e48ad7c3d776cf6f2395c504dc19551268ea7429496726c5d5bf72f9333cba519c21c0000000000000000000000000000000000000000000000000000000000000000100000000";
        let bytes = hex::decode(hex).unwrap();
        let tx = Transaction::consensus_decode(&mut bytes.as_slice()).unwrap();

        let hash_type = TapSighashType::All;
        let input_amounts = [Amount::from_sat(1130279), Amount::from_sat(30000)];
        let input_script_pub_keys = [
            ScriptBuf::from_bytes(
                hex::decode("5120667bdd93c7c029767fd516d2ea292624b938fefefa175ac9f1220cf508963ff3")
                    .unwrap(),
            ),
            ScriptBuf::from_bytes(
                hex::decode("5120667bdd93c7c029767fd516d2ea292624b938fefefa175ac9f1220cf508963ff3")
                    .unwrap(),
            ),
        ];

        // the test transaction actually uses only the key path, so we need to forge the tap leaf hash.
        let mut prng = ChaCha20Rng::seed_from_u64(0);
        let mut random_tap_data = [0u8; 40];
        prng.fill_bytes(&mut random_tap_data);

        let tap_leaf_hash = TapLeafHash::hash(&random_tap_data);

        let tx_preimage_expected = {
            let mut bytes = vec![];
            let mut sighashcache = SighashCache::new(tx.clone());
            sighashcache
                .taproot_encode_signing_data_to(
                    &mut bytes,
                    0,
                    &Prevouts::All(&[
                        TxOut {
                            value: input_amounts[0].clone(),
                            script_pubkey: input_script_pub_keys[0].clone(),
                        },
                        TxOut {
                            value: input_amounts[1].clone(),
                            script_pubkey: input_script_pub_keys[1].clone(),
                        },
                    ]),
                    None,
                    Some((tap_leaf_hash, 0xffffffffu32)),
                    TapSighashType::All,
                )
                .unwrap();

            bytes
        };

        let script = script! {
            { Step1EpochGadget::default() }
            { Step2HashTypeGadget::from_constant(&hash_type) }
            { Step3VersionGadget::from_constant(&tx.version) }
            { Step4LockTimeGadget::from_constant_absolute(&tx.lock_time) }
            OP_CAT4
            if ![TapSighashType::AllPlusAnyoneCanPay, TapSighashType::NonePlusAnyoneCanPay, TapSighashType::SinglePlusAnyoneCanPay].contains(&hash_type) {
                OP_PUSHBYTES_0

                for input in tx.input.iter() {
                    { step5_tx_data_part1_if_not_anyonecanpay::Step1OutPointGadget::from_constant(&input.previous_output) }
                    OP_CAT2
                }

                OP_SHA256

                OP_PUSHBYTES_0

                for amount in input_amounts.iter() {
                    { step5_tx_data_part1_if_not_anyonecanpay::Step2AmountGadget::from_constant(amount) }
                    OP_CAT2
                }
                OP_SHA256

                OP_PUSHBYTES_0

                for script_pub_key in input_script_pub_keys.iter() {
                    { step5_tx_data_part1_if_not_anyonecanpay::Step3ScriptPubKeyGadget::from_constant(script_pub_key)}
                    OP_CAT2
                }
                OP_SHA256

                OP_PUSHBYTES_0

                for input in tx.input.iter() {
                    { step5_tx_data_part1_if_not_anyonecanpay::Step4SequenceGadget::from_constant(&input.sequence) }
                    OP_CAT2
                }
                OP_SHA256

                OP_CAT4

                OP_CAT2
            }
            if [TapSighashType::All, TapSighashType::Default, TapSighashType::AllPlusAnyoneCanPay].contains(&hash_type) {
                { Step6TxPart2GadgetIfNotNoneOrSingle::from_constant(&tx.output) }
                OP_CAT2
            }
            { Step7SpendTypeGadget::from_constant(1, false) }
            if [TapSighashType::AllPlusAnyoneCanPay, TapSighashType::NonePlusAnyoneCanPay, TapSighashType::SinglePlusAnyoneCanPay].contains(&hash_type) {
                { step8_data_input_part_if_anyonecanpay::Step1OutPointGadget::from_constant(&tx.input[0].previous_output) }
                { step8_data_input_part_if_anyonecanpay::Step2AmountGadget::from_constant(&input_amounts[0]) }
                { step8_data_input_part_if_anyonecanpay::Step3ScriptPubKeyGadget::from_constant(&input_script_pub_keys[0]) }
                { step8_data_input_part_if_anyonecanpay::Step4SequenceGadget::from_constant(&tx.input[0].sequence) }
                OP_CAT4
            } else {
                { Step9InputIndexGadgetIfNotAnyOneCanPay::from_constant(0) }
            }
            { Step10AnnexGadgetIfPresent::none() }
            OP_CAT4
            if [TapSighashType::Single, TapSighashType::SinglePlusAnyoneCanPay].contains(&hash_type) {
                { step11_this_output_if_single::Step1AmountGadget::from_constant(&tx.output[0].value) }
                { step11_this_output_if_single::Step2ScriptPubKeyGadget::from_constant(&tx.output[0].script_pubkey) }
                OP_CAT2
            }
            { step12_ext::Step1TapLeafHashGadget::from_constant(&tap_leaf_hash) }
            { step12_ext::Step2KeyVersionGadget::from_constant(0) }
            { step12_ext::Step3CodeSepPosGadget::no_code_sep_executed() }
            OP_CAT3
            OP_CAT2

            { tx_preimage_expected.clone() }
            OP_EQUALVERIFY

            { TapCSVPreImageGadget::from_constant(&tx, &input_amounts, &input_script_pub_keys, 0, &tap_leaf_hash, None, &TapSighashType::All) }
            { tx_preimage_expected }
            OP_EQUAL
        };

        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }
}
