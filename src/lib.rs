pub mod covenant_program;
pub mod covenant_program_nocheck;

/// Modules for some internal structures such as C++-like integers and Bitcoin VI.
pub mod internal_structures;

/// Modules for transaction structures.
pub mod structures;

/// Modules for wrappers that guide constructing complex structures.
pub mod wizards;

/// Modules for some utility functions.
pub mod utils;

/// The covenant script implementation.
pub mod bitcoin_script;

/// Test module
pub mod test;

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
