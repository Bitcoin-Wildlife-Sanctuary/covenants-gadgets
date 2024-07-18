//! The covenant gadget crate implements a number of Bitcoin script gadgets that
//! make it easy for developers to build applications from Bitcoin script.

#![deny(missing_docs)]

/// Modules for covenants, including sequential mode and parallel mode
pub mod covenants;

/// Modules for some internal structures such as C++-like integers and Bitcoin VI.
pub mod internal_structures;

/// Modules for transaction structures.
pub mod structures;

/// Modules for wrappers that guide constructing complex structures.
pub mod wizards;

/// Modules for some utility functions.
pub mod utils;

/// The treepp implementation.
pub(crate) mod treepp {
    pub use bitcoin_script::{define_pushable, script};
    #[cfg(test)]
    pub use bitcoin_scriptexec::execute_script;

    define_pushable!();

    pub use bitcoin::ScriptBuf as Script;
}

use bitcoin::taproot::TaprootSpendInfo;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

use anyhow::Result;
use std::fmt::Debug;

/// The "Nothing Up My Sleeve" (NUMS) point.
pub static SECP256K1_GENERATOR: Lazy<Vec<u8>> = Lazy::new(|| {
    hex::decode("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798").unwrap()
});

/// The dust amount for a P2WSH transaction.
pub const DUST_AMOUNT: u64 = 330;

/// The script map.
pub static SCRIPT_MAPS: OnceLock<Mutex<BTreeMap<&'static str, BTreeMap<usize, treepp::Script>>>> =
    OnceLock::new();

/// The taproot spend info.
pub static TAPROOT_SPEND_INFOS: OnceLock<Mutex<BTreeMap<&'static str, TaprootSpendInfo>>> =
    OnceLock::new();

/// Trait for a covenant program.
pub trait CovenantProgram {
    /// Type of the state for this covenant program.
    type State: Into<treepp::Script> + Debug + Clone;

    /// Type of input (could be an enum).
    type Input: Into<treepp::Script> + Clone;

    /// Unique name for caching.
    const CACHE_NAME: &'static str;

    /// Create an empty state.
    fn new() -> Self::State;

    /// Compute the state hash, which is application-specific.
    fn get_hash(state: &Self::State) -> Vec<u8>;

    /// Get all the scripts of this application.
    fn get_all_scripts() -> BTreeMap<usize, treepp::Script>;

    /// Get the common prefix script.
    fn get_common_prefix() -> treepp::Script;

    /// Run the program to move from the previous state to the new state.
    fn run(id: usize, old_state: &Self::State, input: &Self::Input) -> Result<Self::State>;
}

/// The instruction to simulate the next step.
pub struct SimulationInstruction<T: CovenantProgram> {
    /// The index of the program to be executed.
    pub program_index: usize,
    /// The fee to reserve for the transaction fee.
    pub fee: usize,
    /// The program input.
    pub program_input: T::Input,
}
