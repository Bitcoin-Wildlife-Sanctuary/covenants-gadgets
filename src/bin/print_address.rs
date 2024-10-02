use bitcoin::hashes::Hash;
use bitcoin::opcodes::all::{OP_PUSHBYTES_36, OP_RETURN};
use bitcoin::{Address, Network, ScriptBuf, WScriptHash};
use covenants_gadgets::examples::counter::CounterProgram;
use covenants_gadgets::{get_script_pub_key, CovenantProgram};

fn main() {
    let script_pub_key = get_script_pub_key::<CounterProgram>();

    let program_address =
        Address::from_script(script_pub_key.as_script(), Network::Signet).unwrap();

    let init_state = CounterProgram::new();
    let hash = CounterProgram::get_hash(&init_state);

    let mut bytes = vec![OP_RETURN.to_u8(), OP_PUSHBYTES_36.to_u8()];
    bytes.extend_from_slice(&hash);
    bytes.extend_from_slice(&12u32.to_le_bytes());

    let caboose_address = Address::from_script(
        ScriptBuf::new_p2wsh(&WScriptHash::hash(&bytes)).as_script(),
        Network::Signet,
    )
    .unwrap();

    println!("{}", program_address);
    println!("{}", caboose_address);
}
