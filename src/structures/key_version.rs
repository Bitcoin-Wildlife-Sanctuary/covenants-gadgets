use bitvm::treepp::*;

pub struct KeyVersionGadget;

impl KeyVersionGadget {
    pub fn from_constant(version: u8) -> Script {
        assert_eq!(version, 0);

        script! {
            OP_PUSHBYTES_1 OP_PUSHBYTES_0
        }
    }
}
