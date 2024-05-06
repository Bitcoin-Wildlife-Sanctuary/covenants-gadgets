use bitcoin::TapSighashType;
use bitvm::treepp::*;

pub struct HashTypeGadget;

impl HashTypeGadget {
    pub fn from_constant(hash_type: TapSighashType) -> Script {
        match hash_type {
            TapSighashType::Default => {
                script! {
                    OP_PUSHBYTES_1 OP_PUSHBYTES_0
                }
            }
            TapSighashType::All => {
                script! {
                    OP_PUSHBYTES_1 OP_PUSHBYTES_1
                }
            }
            TapSighashType::None => {
                script! {
                    OP_PUSHBYTES_1 OP_PUSHBYTES_2
                }
            }
            TapSighashType::Single => {
                script! {
                    OP_PUSHBYTES_1 OP_PUSHBYTES_3
                }
            }
            TapSighashType::AllPlusAnyoneCanPay => {
                script! {
                    OP_PUSHBYTES_1 OP_RIGHT
                }
            }
            TapSighashType::NonePlusAnyoneCanPay => {
                script! {
                    OP_PUSHBYTES_1 OP_SIZE
                }
            }
            TapSighashType::SinglePlusAnyoneCanPay => {
                script! {
                    OP_PUSHBYTES_1 OP_INVERT
                }
            }
        }
    }

    pub fn from_provided() -> Script {
        script! {
            OP_DUP OP_PUSHBYTES_1 OP_PUSHBYTES_0 OP_EQUAL
            OP_OVER OP_PUSHBYTES_1 OP_PUSHBYTES_1 OP_EQUAL OP_BOOLOR
            OP_OVER OP_PUSHBYTES_1 OP_PUSHBYTES_2 OP_EQUAL OP_BOOLOR
            OP_OVER OP_PUSHBYTES_1 OP_PUSHBYTES_3 OP_EQUAL OP_BOOLOR
            OP_OVER OP_PUSHBYTES_1 OP_RIGHT OP_EQUAL OP_BOOLOR
            OP_OVER OP_PUSHBYTES_1 OP_SIZE OP_EQUAL OP_BOOLOR
            OP_OVER OP_PUSHBYTES_1 OP_INVERT OP_EQUAL OP_BOOLOR
            OP_VERIFY
        }
    }
}
