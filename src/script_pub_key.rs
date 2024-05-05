use bitcoin::opcodes::all::{OP_PUSHBYTES_1, OP_PUSHBYTES_20, OP_PUSHBYTES_32};
use bitcoin::opcodes::OP_0;
use bitvm::treepp::*;

pub struct ScriptPubKeyGadget;

impl ScriptPubKeyGadget {
    pub fn p2wpkh_from_constant_hash(pkhash: &[u8]) -> Script {
        assert_eq!(pkhash.len(), 20);

        let mut script = vec![OP_0.to_u8(), OP_PUSHBYTES_20.to_u8()];
        script.extend_from_slice(pkhash);
        Script::from_bytes(script)
    }

    pub fn p2wsh_from_constant_hash(script_hash: &[u8]) -> Script {
        assert_eq!(script_hash.len(), 32);

        let mut script = vec![OP_0.to_u8(), OP_PUSHBYTES_32.to_u8()];
        script.extend_from_slice(script_hash);
        Script::from_bytes(script)
    }

    pub fn p2tr_from_public_key(public_key: &[u8]) -> Script {
        assert_eq!(public_key.len(), 32);

        let mut script = vec![OP_PUSHBYTES_1.to_u8(), OP_PUSHBYTES_32.to_u8()];
        script.extend_from_slice(public_key);
        Script::from_bytes(script)
    }
}
