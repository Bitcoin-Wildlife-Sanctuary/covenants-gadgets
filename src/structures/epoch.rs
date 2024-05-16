use bitvm::treepp::*;

pub struct EpochGadget;

impl EpochGadget {
    pub fn default() -> Script {
        script! {
            OP_PUSHBYTES_1 OP_PUSHBYTES_0
        }
    }
}
