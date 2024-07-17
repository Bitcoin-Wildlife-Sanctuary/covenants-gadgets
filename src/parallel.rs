//! The covenant gadget crate implements a number of Bitcoin script gadgets that
//! make it easy for developers to build applications from Bitcoin script.

#![deny(missing_docs)]

use crate::{treepp::*, SimulationInstruction};

use crate::bitcoin_script::covenant;
use crate::structures::tagged_hash::get_hashed_tag;
use crate::CovenantProgram;
use crate::{DUST_AMOUNT, SCRIPT_MAPS, SECP256K1_GENERATOR, TAPROOT_SPEND_INFOS};
use bitcoin::absolute::LockTime;
use bitcoin::hashes::Hash;
use bitcoin::key::UntweakedPublicKey;
use bitcoin::opcodes::all::{OP_PUSHBYTES_36, OP_PUSHBYTES_68, OP_RETURN};
use bitcoin::sighash::{Prevouts, SighashCache};
use bitcoin::taproot::{LeafVersion, TaprootBuilder, TaprootSpendInfo};
use bitcoin::transaction::Version;
use bitcoin::{
    Amount, OutPoint, ScriptBuf, Sequence, TapLeafHash, TapSighashType, Transaction, TxIn, TxOut,
    Txid, WScriptHash, Witness, WitnessProgram,
};
use bitcoin_scriptexec::{convert_to_witness, TxTemplate};
use bitcoin_simulator::database::Database;
use bitcoin_simulator::policy::Policy;
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sha2::Digest;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Mutex;

/// Information necessary to create the new transaction
pub struct CovenantInput {
    /// The randomizer used in the previous caboose (for the Schnorr trick to work).
    pub old_randomizer: u32,
    /// The balance carried by the old state.
    pub old_balance: u64,
    /// The txid of the old state.
    pub old_txid: Txid,

    /// The first input's outpoint of the transaction with txid.
    pub input_outpoint1: OutPoint,
    /// The second input's outpoint of the transaction with txid.
    /// Note: the second input is optional.
    pub input_outpoint2: Option<OutPoint>,

    /// The second input in the new transaction, used to deposit more money into the program.
    /// Note: The witness must be provided for this input.
    pub optional_deposit_input: Option<TxIn>,

    /// The balance of the new state, which needs to be smaller than the old balance plus the deposit,
    /// but does not need to equal (some sats will be used to cover the transaction fee).
    pub new_balance: u64,
}

/// Initialize the taproot spend info.
pub fn compute_taproot_spend_info<T: CovenantProgram>() -> TaprootSpendInfo {
    let secp = bitcoin::secp256k1::Secp256k1::new();
    let internal_key = UntweakedPublicKey::from(
        bitcoin::secp256k1::PublicKey::from_str(
            "0250929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0",
        )
        .unwrap(),
    );

    let mut map = SCRIPT_MAPS
        .get_or_init(|| Mutex::new(BTreeMap::new()))
        .lock()
        .unwrap();
    let scripts = map.entry(T::CACHE_NAME).or_insert_with(T::get_all_scripts);

    let common_prefix = T::get_common_prefix();

    let taproot_builder = TaprootBuilder::with_huffman_tree(scripts.iter().map(|(_, script)| {
        (
            1,
            script! {
                { covenant(false) }
                { common_prefix.clone() }
                { script.clone() }
            },
        )
    }))
    .unwrap();

    let taproot_spend_info = taproot_builder.finalize(&secp, internal_key).unwrap();
    taproot_spend_info
}

/// Compute the script pub key.
pub fn get_script_pub_key<T: CovenantProgram>() -> ScriptBuf {
    let secp = bitcoin::secp256k1::Secp256k1::new();
    let internal_key = UntweakedPublicKey::from(
        bitcoin::secp256k1::PublicKey::from_str(
            "0250929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0",
        )
        .unwrap(),
    );

    let mut map = TAPROOT_SPEND_INFOS
        .get_or_init(|| Mutex::new(BTreeMap::new()))
        .lock()
        .unwrap();
    let taproot_spend_info = map
        .entry(T::CACHE_NAME)
        .or_insert_with(compute_taproot_spend_info::<T>);

    let witness_program =
        WitnessProgram::p2tr(&secp, internal_key, taproot_spend_info.merkle_root());
    let script_pub_key = ScriptBuf::new_witness_program(&witness_program);
    script_pub_key
}

/// Compute the control block and script.
pub fn get_control_block_and_script<T: CovenantProgram>(id: usize) -> (Vec<u8>, Script) {
    let mut map: std::sync::MutexGuard<BTreeMap<&str, TaprootSpendInfo>> = TAPROOT_SPEND_INFOS
        .get_or_init(|| Mutex::new(BTreeMap::new()))
        .lock()
        .unwrap();
    let taproot_spend_info = map
        .entry(T::CACHE_NAME)
        .or_insert_with(compute_taproot_spend_info::<T>);

    let mut map2 = SCRIPT_MAPS
        .get_or_init(|| Mutex::new(BTreeMap::new()))
        .lock()
        .unwrap();
    let script = map2
        .entry(T::CACHE_NAME)
        .or_insert_with(T::get_all_scripts)
        .get(&id)
        .unwrap()
        .clone();

    let common_prefix = T::get_common_prefix();

    let script = script! {
        { covenant(false) }
        { common_prefix.clone() }
        { script.clone() }
    };

    let mut control_block_bytes = Vec::new();
    taproot_spend_info
        .control_block(&(script.clone(), LeafVersion::TapScript))
        .unwrap()
        .encode(&mut control_block_bytes)
        .unwrap();

    (control_block_bytes, script)
}

/// Generate the new transaction and return the new transaction as well as the randomizer
pub fn get_tx<T: CovenantProgram>(
    info: &CovenantInput,
    id: usize,
    old_state: &T::State,
    new_state: &T::State,
    input: &T::Input,
) -> (TxTemplate, u32) {
    let script_pub_key = get_script_pub_key::<T>();
    let (control_block_bytes, script) = get_control_block_and_script::<T>(id);

    let tap_leaf_hash = TapLeafHash::from_script(&script, LeafVersion::TapScript);

    // Initialize a new transaction.
    let mut tx = Transaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO,
        input: vec![],
        output: vec![],
    };

    // Push the previous program as the first input, with the witness left blank as a placeholder.
    tx.input.push(TxIn {
        previous_output: OutPoint::new(info.old_txid.clone(), 0),
        script_sig: ScriptBuf::new(),
        sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
        witness: Witness::new(), // placeholder
    });

    // If there is an optional deposit input, include it as well.
    if let Some(input) = &info.optional_deposit_input {
        tx.input.push(input.clone());
    }

    // Push the first output, which is the new program (and the only change is in the balance).
    tx.output.push(TxOut {
        value: Amount::from_sat(info.new_balance),
        script_pubkey: script_pub_key.clone(),
    });

    let old_state_hash = T::get_hash(old_state);
    let new_state_hash = T::get_hash(new_state);

    // Start the search of a working randomizer from 0.
    let mut randomizer = 0u32;

    // Initialize a placeholder for e, which is the signature element "e" in Schnorr signature.
    // Finding e relies on trial-and-error. Specifically, e is a tagged hash of the signature preimage,
    // and the signature preimage is calculated by serializing the transaction in a specific way.
    let e;
    loop {
        let mut script_bytes = vec![OP_RETURN.to_u8(), OP_PUSHBYTES_68.to_u8()];
        script_bytes.extend_from_slice(&old_state_hash);
        script_bytes.extend_from_slice(&new_state_hash);
        script_bytes.extend_from_slice(&randomizer.to_le_bytes());

        // Generate the corresponding caboose with the new counter.
        let witness_program = WitnessProgram::p2wsh(&ScriptBuf::from_bytes(script_bytes));

        // Temporarily insert this output.
        // If this output doesn't work, in a later step, we will revert the insertion and remove this
        // output from the transaction.
        tx.output.push(TxOut {
            value: Amount::from_sat(DUST_AMOUNT),
            script_pubkey: ScriptBuf::new_witness_program(&witness_program),
        });

        // Initialize the SighashCache object for computing the signature preimage.
        let mut sighashcache = SighashCache::new(tx.clone());

        // Compute the taproot hash assuming AllPlusAnyoneCanPay.
        let hash = AsRef::<[u8]>::as_ref(
            &sighashcache
                .taproot_script_spend_signature_hash(
                    0,
                    &Prevouts::One(
                        0,
                        &TxOut {
                            value: Amount::from_sat(info.old_balance),
                            script_pubkey: script_pub_key.clone(),
                        },
                    ),
                    tap_leaf_hash,
                    TapSighashType::AllPlusAnyoneCanPay,
                )
                .unwrap(),
        )
        .to_vec();

        // Compute the tagged hash of the signature preimage.
        let bip340challenge_prefix = get_hashed_tag("BIP0340/challenge");
        let mut sha256 = sha2::Sha256::new();
        Digest::update(&mut sha256, &bip340challenge_prefix);
        Digest::update(&mut sha256, &bip340challenge_prefix);
        Digest::update(&mut sha256, SECP256K1_GENERATOR.as_slice());
        Digest::update(&mut sha256, SECP256K1_GENERATOR.as_slice());
        Digest::update(&mut sha256, hash);
        let e_expected = sha256.finalize().to_vec();

        // If the signature preimage ends with 0x01 (which is consistent to the Schnorr trick),
        // we will accept this randomizer.
        //
        // Note: this is in fact not a strict requirement that it needs to be ending at 0x01.
        // Nevertheless, requiring so makes sure that we can avoid the corner case (ending at 0xff),
        // and it is consistent with the Schnorr trick article.
        if e_expected[31] == 0x01 {
            e = Some(e_expected);
            break;
        } else {
            // Remove the nonfunctional output and retry.
            tx.output.pop().unwrap();
            randomizer += 1;
        }
    }

    // now start preparing the witness
    let mut script_execution_witness = Vec::<Vec<u8>>::new();

    // new balance (8 bytes)
    script_execution_witness.push(info.new_balance.to_le_bytes().to_vec());

    // this script's scriptpubkey (34 bytes)
    script_execution_witness.push(script_pub_key.to_bytes());

    // the new counter hash
    script_execution_witness.push(new_state_hash.clone());

    // the old counter hash
    script_execution_witness.push(old_state_hash.clone());

    // the randomizer (4 bytes)
    script_execution_witness.push(randomizer.to_le_bytes().to_vec());

    // previous tx's txid (32 bytes)
    script_execution_witness.push(AsRef::<[u8]>::as_ref(&info.old_txid).to_vec());

    // previous balance (8 bytes)
    script_execution_witness.push(info.old_balance.to_le_bytes().to_vec());

    // tap leaf hash (32 bytes)
    script_execution_witness.push(AsRef::<[u8]>::as_ref(&tap_leaf_hash).to_vec());

    // the sha256 without the last byte (31 bytes)
    script_execution_witness.push(e.unwrap()[0..31].to_vec());

    // application-specific witnesses
    let old_state_in_script: Script = old_state.clone().into();
    let new_state_in_script: Script = new_state.clone().into();
    let input_in_script: Script = input.clone().into();
    let application_witness = convert_to_witness(script! {
        { old_state_in_script }
        { new_state_in_script }
        { input_in_script }
    })
    .unwrap();

    script_execution_witness.extend_from_slice(&application_witness);

    // Construct the witness that will be included in the TxIn.
    let mut script_tx_witness = Witness::new();
    // all the initial stack elements
    for elem in script_execution_witness.iter() {
        script_tx_witness.push(elem);
    }
    // the full script
    script_tx_witness.push(script);
    // the control block bytes
    script_tx_witness.push(control_block_bytes);

    // Include the witness in the TxIn.
    tx.input[0].witness = script_tx_witness;

    // Prepare the TxTemplate.
    let tx_template = TxTemplate {
        tx,
        prevouts: vec![TxOut {
            value: Amount::from_sat(info.old_balance),
            script_pubkey: script_pub_key.clone(),
        }],
        input_idx: 0,
        taproot_annex_scriptleaf: Some((tap_leaf_hash.clone(), None)),
    };

    (tx_template, randomizer)
}

/// run simulation test parallel
pub fn simulation_test<T: CovenantProgram>(
    repeat: usize,
    test_generator: &mut impl FnMut(&T::State) -> Option<SimulationInstruction<T>>,
) {
    let policy = Policy::default().set_fee(7).set_max_tx_weight(400000);

    let prng = Rc::new(RefCell::new(ChaCha20Rng::seed_from_u64(0)));
    let get_rand_txid = || {
        let mut bytes = [0u8; 20];
        prng.borrow_mut().fill_bytes(&mut bytes);
        Txid::hash(&bytes)
    };

    let db = Database::connect_temporary_database().unwrap();

    let init_state = T::new();
    let init_old_state_hash = T::get_hash(&T::new());
    let init_state_hash = T::get_hash(&init_state);
    let script_pub_key = get_script_pub_key::<T>();

    let init_randomizer = 12u32;

    let mut script_bytes = vec![OP_RETURN.to_u8(), OP_PUSHBYTES_68.to_u8()];
    script_bytes.extend_from_slice(&init_old_state_hash);
    script_bytes.extend_from_slice(&init_state_hash);
    script_bytes.extend_from_slice(&init_randomizer.to_le_bytes());

    let prev_witness_program = WitnessProgram::p2wsh(&ScriptBuf::from_bytes(script_bytes));

    // initialize the counter and accept it unconditionally
    let init_tx = Transaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: get_rand_txid(),
                vout: 0,
            },
            script_sig: ScriptBuf::default(),
            sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
            witness: Witness::new(),
        }],
        output: vec![
            TxOut {
                value: Amount::from_sat(1_000_000_000),
                script_pubkey: script_pub_key.clone(),
            },
            TxOut {
                value: Amount::from_sat(DUST_AMOUNT),
                script_pubkey: ScriptBuf::new_witness_program(&prev_witness_program),
            },
        ],
    };

    // Ignore whether the TxIn is valid, make the outputs available in the network.
    db.insert_transaction_unconditionally(&init_tx).unwrap();

    // Prepare the trivial script, which is used for testing purposes to deposit more money
    // into the program.
    let trivial_p2wsh_script = script! {
        OP_TRUE
    };

    let trivial_p2wsh_script_pubkey =
        ScriptBuf::new_p2wsh(&WScriptHash::hash(trivial_p2wsh_script.as_bytes()));

    let mut trivial_p2wsh_witness = Witness::new();
    trivial_p2wsh_witness.push([]);
    trivial_p2wsh_witness.push(trivial_p2wsh_script);

    // Initialize the state.
    let mut old_state = init_state;
    let mut old_randomizer = init_randomizer;
    let mut old_balance = 1_000_000_000u64;
    let mut old_txid = init_tx.compute_txid();

    let mut old_tx_outpoint1 = init_tx.input[0].previous_output;
    let mut old_tx_outpoint2 = None;

    #[cfg(feature = "debug")]
    eprintln!("{:?}", old_state);

    for _ in 0..repeat {
        let mut has_deposit_input = prng.borrow_mut().gen::<bool>();

        if old_balance < 700_000u64 {
            has_deposit_input = true;
        }

        // If there is a deposit input
        let deposit_input = if has_deposit_input {
            let fee_tx = Transaction {
                version: Version::TWO,
                lock_time: LockTime::ZERO,
                input: vec![TxIn {
                    previous_output: OutPoint {
                        txid: get_rand_txid(),
                        vout: 0xffffffffu32,
                    },
                    script_sig: ScriptBuf::new(),
                    sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
                    witness: Witness::new(),
                }], // a random input is needed to avoid TXID collision.
                output: vec![TxOut {
                    value: Amount::from_sat(123_456_000),
                    script_pubkey: trivial_p2wsh_script_pubkey.clone(),
                }],
            };

            db.insert_transaction_unconditionally(&fee_tx).unwrap();

            Some(TxIn {
                previous_output: OutPoint {
                    txid: fee_tx.compute_txid(),
                    vout: 0,
                },
                script_sig: ScriptBuf::new(),
                sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
                witness: trivial_p2wsh_witness.clone(),
            })
        } else {
            None
        };

        let next_step = test_generator(&old_state);
        if next_step.is_none() {
            return;
        }
        let SimulationInstruction::<T> {
            program_index: id,
            program_input: input,
            fee,
        } = next_step.unwrap();

        let mut new_balance = old_balance;
        if deposit_input.is_some() {
            new_balance += 123_456_000;
        }
        new_balance -= fee as u64; // as for transaction fee
        new_balance -= DUST_AMOUNT;

        let info = CovenantInput {
            old_randomizer,
            old_balance,
            old_txid: old_txid.clone(),
            input_outpoint1: old_tx_outpoint1.clone(),
            input_outpoint2: old_tx_outpoint2.clone(),
            optional_deposit_input: deposit_input,
            new_balance,
        };

        let new_state = T::run(id, &old_state, &input).unwrap();

        let (tx_template, randomizer) = get_tx::<T>(&info, id, &old_state, &new_state, &input);

        // Check if the new transaction conforms to the requirement.
        // If so, insert this transaction unconditionally.
        db.verify_transaction(&tx_template.tx).unwrap();
        db.check_fees(&tx_template.tx, &policy).unwrap();
        db.insert_transaction_unconditionally(&tx_template.tx)
            .unwrap();

        // Update the local state.
        old_state = new_state;
        old_randomizer = randomizer;
        old_balance = new_balance;
        old_txid = tx_template.tx.compute_txid();

        #[cfg(feature = "debug")]
        eprintln!("{:?}", old_state);

        old_tx_outpoint1 = tx_template.tx.input[0].previous_output;
        old_tx_outpoint2 = tx_template
            .tx
            .input
            .get(1)
            .and_then(|x| Some(x.previous_output.clone()));
    }
}
