//! The covenant gadget crate implements a number of Bitcoin script gadgets that
//! make it easy for developers to build applications from Bitcoin script.

#![deny(missing_docs)]

/// Modules for some internal structures such as C++-like integers and Bitcoin VI.
pub mod internal_structures;

/// Modules for transaction structures.
pub mod structures;

/// Modules for wrappers that guide constructing complex structures.
pub mod wizards;

/// Modules for some utility functions.
pub mod utils;
