use bitvm::treepp::*;

pub struct ScriptSigGadget;

impl ScriptSigGadget {
    pub fn segregated_witness() -> Script {
        script! {
            OP_PUSHBYTES_1 OP_PUSHBYTES_0
        }
    }
}
