use crate::treepp::*;
use crate::{get_script_pub_key, get_tx, CovenantInput, CovenantProgram, DUST_AMOUNT};
use bitcoin::absolute::LockTime;
use bitcoin::hashes::Hash;
use bitcoin::opcodes::all::{OP_PUSHBYTES_36, OP_RETURN};
use bitcoin::transaction::Version;
use bitcoin::{
    Amount, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Txid, WScriptHash, Witness,
    WitnessProgram,
};
use bitcoin_simulator::database::Database;
use bitcoin_simulator::policy::Policy;
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use std::cell::RefCell;
use std::rc::Rc;

/// The instruction to simulate the next step.
pub struct SimulationInstruction<T: CovenantProgram> {
    /// The index of the program to be executed.
    pub program_index: usize,
    /// The fee to reserve for the transaction fee.
    pub fee: usize,
    /// The program input.
    pub program_input: T::Input,
}

/// Run simulation test.
pub fn simulation_test<T: CovenantProgram>(
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
    let init_state_hash = T::get_hash(&init_state);
    let script_pub_key = get_script_pub_key::<T>();

    let init_randomizer = 12u32;

    let mut script_bytes = vec![OP_RETURN.to_u8(), OP_PUSHBYTES_36.to_u8()];
    script_bytes.extend_from_slice(&init_state_hash);
    script_bytes.extend_from_slice(&init_randomizer.to_le_bytes());

    let prev_witness_program = WitnessProgram::p2wsh(&ScriptBuf::from_bytes(script_bytes));

    // initialize the counter and accept it unconditionally
    let init_tx = Transaction {
        version: Version::ONE,
        lock_time: LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: get_rand_txid(),
                vout: 0,
            },
            script_sig: ScriptBuf::default(),
            sequence: Sequence::default(),
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

    eprintln!("{:?}", old_state);

    for _ in 0..100 {
        let mut has_deposit_input = prng.borrow_mut().gen::<bool>();

        if old_balance < 700_000u64 {
            has_deposit_input = true;
        }

        // If there is a deposit input
        let deposit_input = if has_deposit_input {
            let fee_tx = Transaction {
                version: Version::ONE,
                lock_time: LockTime::ZERO,
                input: vec![TxIn {
                    previous_output: OutPoint {
                        txid: get_rand_txid(),
                        vout: 0xffffffffu32,
                    },
                    script_sig: ScriptBuf::new(),
                    sequence: Sequence::default(),
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
                sequence: Sequence::default(),
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

        eprintln!("{:?}", old_state);

        old_tx_outpoint1 = tx_template.tx.input[0].previous_output;
        old_tx_outpoint2 = tx_template
            .tx
            .input
            .get(1)
            .and_then(|x| Some(x.previous_output.clone()));
    }
}
