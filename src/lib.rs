//! The covenant gadget crate implements a number of Bitcoin script gadgets that
//! make it easy for developers to build applications from Bitcoin script.

#![deny(missing_docs)]

pub(crate) mod treepp {
    pub use bitcoin_script::{define_pushable, script};
    #[cfg(test)]
    pub use bitcoin_scriptexec::execute_script;

    define_pushable!();
    pub use bitcoin::ScriptBuf as Script;
}

/// Modules for some internal structures such as C++-like integers and Bitcoin VI.
pub mod internal_structures;

/// Modules for transaction structures.
pub mod structures;

/// Modules for wrappers that guide constructing complex structures.
pub mod wizards;

/// Modules for some utility functions.
pub mod utils;
