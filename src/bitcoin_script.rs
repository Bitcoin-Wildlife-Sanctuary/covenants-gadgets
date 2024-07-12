use crate::structures::tagged_hash::{HashTag, TaggedHashGadget};
use crate::treepp::*;
use crate::utils::pseudo::{OP_CAT2, OP_CAT3, OP_CAT4, OP_HINT};
use crate::wizards::{tap_csv_preimage, tx};
use crate::DUST_AMOUNT;
use crate::SECP256K1_GENERATOR;
use bitcoin::absolute::LockTime;
use bitcoin::transaction::Version;
use bitcoin::{Amount, Sequence, TapSighashType};

/// Step 1: Create the beginning part of the preimage.
///
/// Output:
/// - preimage_head
///
pub fn step1() -> Script {
    script! {
        // For more information about the construction of the Tap CheckSigVerify Preimage, please
        // check out the `covenants-gadgets` repository.

        { tap_csv_preimage::Step1EpochGadget::default() }
        { tap_csv_preimage::Step2HashTypeGadget::from_constant(&TapSighashType::AllPlusAnyoneCanPay) }
        { tap_csv_preimage::Step3VersionGadget::from_constant(&Version::TWO) }
        { tap_csv_preimage::Step4LockTimeGadget::from_constant_absolute(&LockTime::ZERO) }
        OP_CAT4
    }
}

/// Step 2: Assemble the first output, which is the program itself, with the new balance.
///
/// Hint:
/// - new balance
/// - script pubkey
///
/// Input:
/// - preimage_head
///
/// Output:
/// - preimage_head
/// - pubkey
/// - first_output
/// - dust for second_output
///
pub fn step2() -> Script {
    script! {
        // get a hint: new balance (8 bytes)
        OP_HINT
        OP_SIZE 8 OP_EQUALVERIFY

        // get a hint: this script's scriptpubkey (34 bytes)
        OP_HINT
        OP_SIZE 34 OP_EQUALVERIFY

        // save pubkey to the altstack
        OP_DUP OP_TOALTSTACK

        OP_PUSHBYTES_1 OP_PUSHBYTES_34
        OP_SWAP OP_CAT3

        OP_FROMALTSTACK OP_SWAP

        // CAT dust amount
        OP_PUSHBYTES_8 OP_PUSHBYTES_74 OP_PUSHBYTES_1 OP_PUSHBYTES_0 OP_PUSHBYTES_0
        OP_PUSHBYTES_0 OP_PUSHBYTES_0 OP_PUSHBYTES_0 OP_PUSHBYTES_0
        OP_CAT
    }
}

/// Step 3: deal with the second output via the new and old state hash, computing the new state's script.
///
/// Hint:
/// - new_state_hash
/// - old_state_hash
///
/// Input:
/// - preimage_head
/// - pubkey
/// - first_output
/// - dust for second_output
///
/// Output:
/// - pubkey
/// - old_state_hash
/// - preimage_head | Hash(first output | second_output)
///
/// Altstack:
/// - new_state_hash
/// - old_state_hash
///
pub fn step3() -> Script {
    script! {
        // script hash header
        OP_PUSHBYTES_2 OP_RETURN OP_PUSHBYTES_36

        // get a hint: the new state hash
        OP_HINT
        OP_SIZE 32 OP_EQUALVERIFY
        // save the new state hash to the altstack
        OP_DUP OP_TOALTSTACK

        // get a hint: the old state hash
        OP_HINT
        OP_SIZE 32 OP_EQUALVERIFY
        // save the old state hash in the altstack for later use
        OP_DUP OP_TOALTSTACK
        OP_TOALTSTACK

        // get a hint: the randomizer for this transaction (4 bytes)
        OP_HINT
        OP_SIZE 4 OP_EQUALVERIFY
        OP_CAT3

        OP_SHA256

        OP_PUSHBYTES_3 OP_PUSHBYTES_34 OP_PUSHBYTES_0 OP_PUSHBYTES_32
        OP_SWAP OP_CAT3

        OP_SHA256
        OP_ROT OP_SWAP OP_CAT2

        OP_FROMALTSTACK OP_SWAP
    }
}

/// Step 4: provide the original data of the input.
///
/// Hint:
/// - old_txid
/// - old_amount
///
/// Input:
/// - pubkey
/// - old_state_hash
/// - preimage_head | Hash(first output | second_output)
///
/// Output:
/// - pubkey
/// - old_state_hash
/// - old_amount
/// - old_txid
/// - preimage_head | Hash(first output | second_output) | this_input
///
/// Altstack:
/// - new_state_hash
/// - old_state_hash
///
pub fn step4() -> Script {
    script! {
        { tap_csv_preimage::Step7SpendTypeGadget::from_constant(1, false) } OP_CAT2

        // get a hint: previous tx's txid
        OP_HINT
        OP_SIZE 32 OP_EQUALVERIFY

        // save a copy to altstack
        OP_DUP OP_TOALTSTACK

        // require the output index be 0
        { tap_csv_preimage::step8_data_input_part_if_anyonecanpay::step1_outpoint::Step2IndexGadget::from_constant(0) }
        OP_CAT3

        // get a hint: previous tx's amount
        OP_HINT
        OP_SIZE 8 OP_EQUALVERIFY
        OP_DUP OP_TOALTSTACK
        OP_CAT2

        // add the script pub key
        2 OP_PICK
        OP_PUSHBYTES_1 OP_PUSHBYTES_34 OP_SWAP
        OP_CAT3

        // require the input sequence number be 0xfffffffd
        { tap_csv_preimage::step8_data_input_part_if_anyonecanpay::Step4SequenceGadget::from_constant(&Sequence::ENABLE_RBF_NO_LOCKTIME) }
        OP_CAT2

        OP_FROMALTSTACK OP_SWAP
        OP_FROMALTSTACK OP_SWAP
    }
}

/// Step 5: provide the extension data.
///
/// Hint:
/// - tap leaf hash
///
/// Input:
/// - pubkey
/// - old_state_hash
/// - old_amount
/// - old_txid
/// - preimage_head | Hash(first output | second_output) | this_input
///
/// Output:
/// - pubkey
/// - old_state_hash
/// - old_amount
/// - old_txid
/// - preimage_head | Hash(first output | second_output) | this_input | ext
///
/// Altstack:
/// - new_state_hash
/// - old_state_hash
///
pub fn step5() -> Script {
    script! {
        // get a hint: tap leaf hash
        OP_HINT
        OP_SIZE 32 OP_EQUALVERIFY

        { tap_csv_preimage::step12_ext::Step2KeyVersionGadget::from_constant(0) }
        { tap_csv_preimage::step12_ext::Step3CodeSepPosGadget::no_code_sep_executed() }
        OP_CAT4
    }
}

/// Step 6: verify the reflection using the Schnorr trick.
///
/// Hint:
/// - the SHA256 BIP-340 challenge hash without the last byte (which should be 0x01)
///
/// Input:
/// - pubkey
/// - old_state_hash
/// - old_amount
/// - old_txid
/// - preimage_head | Hash(first output | second_output) | this_input | ext
///
/// Output:
/// - pubkey
/// - old_state_hash
/// - old_amount
/// - old_txid
///
/// Altstack:
/// - new_state_hash
/// - old_state_hash
///
/// The script fails if the preimage doesn't match the transaction.
///
pub fn step6() -> Script {
    // Obtain the secp256k1 dummy generator, which would be point R in the signature, as well as
    // the public key.
    let secp256k1_generator = SECP256K1_GENERATOR.clone();

    script! {
        { TaggedHashGadget::from_provided(&HashTag::TapSighash) }

        { secp256k1_generator.clone() }
        OP_DUP OP_TOALTSTACK
        OP_DUP OP_TOALTSTACK

        OP_DUP OP_ROT OP_CAT3

        { TaggedHashGadget::from_provided(&HashTag::BIP340Challenge) }

        // get a hint: the sha256 without the last byte
        OP_HINT
        OP_SIZE 31 OP_EQUALVERIFY

        OP_DUP { 1 } OP_CAT
        OP_ROT OP_EQUALVERIFY

        OP_FROMALTSTACK OP_SWAP

        OP_PUSHBYTES_2 OP_PUSHBYTES_2 OP_RIGHT
        OP_CAT3

        OP_FROMALTSTACK
        OP_CHECKSIGVERIFY
    }
}

/// Step 7: fill in the old transaction's version and input.
///
/// Below are all related to the old transaction.
///
/// Hint:
/// - first input's outpoint
/// - second input's outpoint (which can be an empty string if there is no second input)
///
/// Input:
/// - pubkey
/// - old_state_hash
/// - old_amount
/// - old_txid
///
/// Output:
/// - pubkey
/// - old_state_hash
/// - old_amount
/// - old_txid
/// - version | inputs
///
/// Altstack:
/// - new_state_hash
/// - old_state_hash
///
pub fn step7() -> Script {
    script! {
        { tx::Step1VersionGadget::from_constant(&Version::TWO) }

        // Below all are related to the old transaction.

        // get a hint: first input's outpoint
        OP_HINT
        OP_SIZE 36 OP_EQUALVERIFY

        // get a hint: second input's outpoint (an empty string if the second input is not present)
        OP_HINT
        OP_SIZE 0 OP_EQUAL OP_TOALTSTACK
        OP_SIZE 36 OP_EQUAL OP_FROMALTSTACK OP_BOOLOR OP_VERIFY

        OP_SIZE 0 OP_EQUAL
        OP_IF
            OP_DROP
            OP_PUSHBYTES_5 OP_PUSHBYTES_0 OP_RETURN_253 OP_INVALIDOPCODE OP_INVALIDOPCODE OP_INVALIDOPCODE
            OP_CAT
            { tx::Step2InCounterGadget::from_constant(1) }
        OP_ELSE
            OP_TOALTSTACK
            OP_PUSHBYTES_5 OP_PUSHBYTES_0 OP_RETURN_253 OP_INVALIDOPCODE OP_INVALIDOPCODE OP_INVALIDOPCODE
            OP_DUP
            OP_FROMALTSTACK OP_SWAP
            OP_CAT4
            { tx::Step2InCounterGadget::from_constant(2) }
        OP_ENDIF
        OP_SWAP OP_CAT
        OP_CAT2
    }
}

/// Step 8: fill in the old transaction's output and locktime.
///
/// Hint:
/// - old_randomizer
///
/// Input:
/// - pubkey
/// - old_state_hash
/// - old_amount
/// - old_txid
/// - version | inputs
///
/// Output:
/// - old_txid
/// - version | inputs | output | locktime
///
/// Altstack:
/// - new_state_hash
/// - old_state_hash
///
pub fn step8() -> Script {
    script! {
        { tx::Step4OutCounterGadget::from_constant(2) }
        OP_CAT2

        // get the previous amount
        2 OP_ROLL
        OP_CAT2

        // get the script pub key
        3 OP_ROLL
        OP_PUSHBYTES_1 OP_PUSHBYTES_34 OP_SWAP
        OP_CAT3

        { tx::step5_output::Step1AmountGadget::from_constant(&Amount::from_sat(DUST_AMOUNT)) }
        OP_CAT2

        // push the script hash header
        OP_PUSHBYTES_2 OP_RETURN OP_PUSHBYTES_36
        3 OP_ROLL

        // get a hint: the randomizer for previous transaction (4 bytes)
        OP_HINT
        OP_SIZE 4 OP_EQUALVERIFY
        OP_CAT3
        OP_SHA256

        OP_PUSHBYTES_3 OP_PUSHBYTES_34 OP_PUSHBYTES_0 OP_PUSHBYTES_32
        OP_SWAP OP_CAT3

        { tx::Step6LockTimeGadget::from_constant_absolute(&LockTime::ZERO) }
        OP_CAT2
    }
}

/// Step 9: check against the old txid.
///
/// Hint:
/// - old_randomizer
///
/// Input:
/// - old_txid
/// - version | inputs | output | locktime
///
/// Output:
/// - old_state_hash
/// - new_state_hash
///

pub fn step9() -> Script {
    script! {
        OP_SHA256
        OP_SHA256
        OP_EQUALVERIFY

        OP_FROMALTSTACK OP_FROMALTSTACK
    }
}

/// Implementation of a standard covenant.
pub fn covenant() -> Script {
    script! {
        step1
        // [..., preimage_head ]

        step2
        // [..., preimage_head, pubkey, first_output | dust ]

        step3
        // [..., pubkey, old_state_hash, preimage_head | Hash(first_output | second_output) ]

        step4
        // [..., pubkey, old_state_hash, old_amount, old_txid, preimage_head | Hash(first_output | second_output) | this_input ]

        step5
        // [..., pubkey, old_state_hash, old_amount, old_txid, preimage_head | Hash(first_output | second_output) | this_input | ext ]

        step6
        // checksigverify done
        // [..., pubkey, old_state_hash, old_amount, old_txid ]

        step7
        // [..., pubkey, old_state_hash, old_amount, old_txid, version | inputs ]

        step8
        // [..., pubkey, old_state_hash, old_amount, old_txid, version | inputs | output | locktime ]

        step9
        // [old_state_hash, new_state_hash]
    }
}
