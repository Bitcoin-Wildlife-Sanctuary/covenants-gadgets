use crate::utils::pseudo::OP_CAT3;
use bitcoin::TxIn;
use bitvm::treepp::*;

pub use crate::structures::script_sig::ScriptSigGadget as Step2ScriptSigGadget;
pub use crate::structures::sequence::SequenceGadget as Step3SequenceGadget;
pub use crate::wizards::outpoint as step1_outpoint;
pub use crate::wizards::outpoint::OutPointGadget as Step1OutPointGadget;

pub struct TxInGadget;

impl TxInGadget {
    pub fn from_constant(tx_in: &TxIn) -> Script {
        assert!(tx_in.script_sig.is_empty());

        script! {
            { Step1OutPointGadget::from_constant(&tx_in.previous_output) }
            { Step2ScriptSigGadget::segregated_witness() }
            { Step3SequenceGadget::from_constant(tx_in.sequence) }
            OP_CAT3
        }
    }
}
