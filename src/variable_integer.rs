use bitvm::treepp::*;

pub struct VariableIntegerGadget;

impl VariableIntegerGadget {
    pub fn encode_small_bitcoin_number() -> Script {
        script! {
            // making sure the number is smaller than 128
            OP_DUP
            { 128 } OP_LESSTHAN OP_VERIFY
        }
    }
}
