pub use crate::internal_structures::variable_length_integer::VariableLengthIntegerGadget as Step2InCounterGadget;
pub use crate::internal_structures::variable_length_integer::VariableLengthIntegerGadget as Step4OutCounterGadget;
pub use crate::structures::locktime::LockTimeGadget as Step6LockTimeGadget;
pub use crate::structures::version::VersionGadget as Step1VersionGadget;
pub use crate::wizards::tx_in as step3_input;
pub use crate::wizards::tx_in::TxInGadget as Step3InputGadget;
pub use crate::wizards::tx_out as step5_output;
pub use crate::wizards::tx_out::TxOutGadget as Step5OutputGadget;
