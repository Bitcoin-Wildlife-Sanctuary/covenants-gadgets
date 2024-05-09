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
