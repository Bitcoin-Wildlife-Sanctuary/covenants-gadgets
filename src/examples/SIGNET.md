This document describes how to do the counter experiment on SIGNET.

### Step 1: install Bitcoin Core inquisition

Download the source code of the inquisition version of Bitcoin Core from [https://github.com/bitcoin-inquisition/bitcoin](https://github.com/bitcoin-inquisition/bitcoin/releases). 
We recommend downloading the source and compiling from it.

Instructions to compile and getting started can be found from the "Getting started" section here:
https://en.bitcoin.it/wiki/Signet

Once it is connected, one can create a new wallet with './bitcoin-cli' and then get a new address:
```sh
./bitcoin-cli --datadir=signet getnewaddress
```

### Step 2: connect to a CAT-ready relay node

Bitcoin client would automatically find a few nearby relay nodes to be used for broadcasting the transaction. However, it is 
very possible that none of these relay nodes are in the new version that supports OP_CAT, and they may refuse to broadcast it 
further (this is the default policy to discourage using OP_SUCCESSXX before they are reassigned).

Finding a relay node with the new version is not sufficient because this relay node may not be reaching the signet block signer 
through a path of relay nodes that all support OP_CAT. 

We reached out to [Taproot Wizards](https://x.com/taprootwizards). Taproot Wizards have set up a public relay node to help these 
transactions safely navigate through the P2P network to finally reach the destination---the signet block signer.

```
signet=1
daemon=1

[signet]
addnode=inquisition.taprootwizards.com
```

The inquisition 27.0 [announcement](https://delvingbitcoin.org/t/bitcoin-inquisition-27-0/883) mentioned an official public inquisition node that is set up by Anthony Towns 
and will support OP_CAT and other signet soft fork gap period issues. However, we suggest to use Taproot Wizards's relay 
node to relieve the official node from other P2P requests that are not relevant to transaction submission---for example, 
synchronizing with the chain---because we would be in more trouble if the signet block signer is down. 

### Step 3: obtain Bitcoin Signet sats for testing

I find two faucets.
- https://signetfaucet.com/
- https://alt.signetfaucet.com

Babylon is also running a faucet.
- https://x.com/babylon_chain/status/1790787732643643575

Note that the sats from the faucet will not arrive immediately. It may appear first as an unconfirmed balance. 

One can find the confirmed balance of the wallet using the following. This needs to wait for about 10 minutes.
```sh
./bitcoin-cli --datadir=signet getbalance
```

One can check, in the meantime, from the mempool explorer: https://mempool.space/signet, to see if the transaction has been included 
in a more global mempool.

### Step 4: prepare a small UTXO transaction to fund the program

We recommend preparing a single UTXO transaction with at least 0.00003 BTC (i.e., 3000 sats) for the experiment. This can be done by using 
the Bitcoin client.

```sh
./bitcoin-cli --datadir=signet sendtoaddress "[a new address in the local wallet]" 0.00003
```

An example is our funding transaction [7592310527184f4496c48324138aa6afb4fb126dc73b99601b3563330c61b0d4](https://mempool.space/signet/tx/7592310527184f4496c48324138aa6afb4fb126dc73b99601b3563330c61b0d4),
which funds 0.00005 (i.e., 5000 sats) to an address tb1q7ywyynypxraygr22qvpwdgrvpekhgx4jzspk77.

### Step 5: initialize the program with the first transaction

We can now use that fund to initialize the first transaction by supplying the following to bitcoin-cli.

- "[{\"txid\":\"7592310527184f4496c48324138aa6afb4fb126dc73b99601b3563330c61b0d4\", \"vout\": 0}]" as the input, where txid is the funding transaction's txid, and vout is the index
  of the output with the desired fund amount (this is not necessarily the first output).
- "[{\"tb1pyt2fue5keveakttttkxyfdxl6rr7t3cphwx56g7eaq0v4y5rzrcs7n26nn\":0.00004500}, {\"tb1qya666a46fnvqtc45rw00rrryfyzda0v0kxwmk588mvrplp0z6hwstgjna4\":0.00000330}]" as the output,
  for which we are sending 4500 sats to tb1pyt2fue5keveakttttkxyfdxl6rr7t3cphwx56g7eaq0v4y5rzrcs7n26nn, the P2TR program, and 330 sats to tb1qya666a46fnvqtc45rw00rrryfyzda0v0kxwmk588mvrplp0z6hwstgjna4,
  the initial state represented by P2WSH. It is important to use 330 sats (which is the dust amount) for the initial state as it is an assumed constant by the script. One can leave about 200
  sats for transaction fees, as this transaction has a vsize of ‎164.25 vBytes and would need ~164.25 sats for the fee.
- additional config parameters "0 false" to indicate a zero locktime and non-replaceable, both of which are not hard requirement but the current script assumes so.

An example is as follows.

```sh
./bitcoin-cli --datadir=signet createrawtransaction "[{\"txid\":\"7592310527184f4496c48324138aa6afb4fb126dc73b99601b3563330c61b0d4\", \"vout\": 0}]" "[{\"tb1pyt2fue5keveakttttkxyfdxl6rr7t3cphwx56g7eaq0v4y5rzrcs7n26nn\":0.00004500}, {\"tb1qya666a46fnvqtc45rw00rrryfyzda0v0kxwmk588mvrplp0z6hwstgjna4\":0.00000330}]" 0 false
```

This would provide the unsigned transaction in hex. We can then sign this transaction. But importantly, we need to make a small modification to the hex.

The default hex from `bitcoin-cli` will start with 02. Our script, however, assumes it would be 01. So, we make the modification, and then use bitcoin-cli to sign this transaction.

```sh
./bitcoin-cli --datadir=signet signrawtransactionwithwallet 0100000001d4b0610c3363351b60993bc76d12fbb4afa68a132483c496444f1827053192750000000000ffffffff02941100000000000022512022d49e6696cb33db2d6b5d8c44b4dfd0c7e5c701bb8d4d23d9e81eca928310f14a010000000000002200202775ad76ba4cd805e2b41b9ef18c644904debd8fb19dbb50e7db061f85e2d5dd00000000
```

We can then broadcast this transaction. Most of the nodes will be happy to relay it since it is just a simple P2WPKH -> P2TR + P2WSH.

```sh
./bitcoin-cli --datadir=signet sendrawtransaction 01000000000101d4b0610c3363351b60993bc76d12fbb4afa68a132483c496444f1827053192750000000000ffffffff02941100000000000022512022d49e6696cb33db2d6b5d8c44b4dfd0c7e5c701bb8d4d23d9e81eca928310f14a010000000000002200202775ad76ba4cd805e2b41b9ef18c644904debd8fb19dbb50e7db061f85e2d5dd024730440220226e2b0c70c5c8895a29ced195c930631fcc3f8a398e2cdd860c5e43fdd71e7902202be9c113b6729eef19254906ea014fdbe9c522188afd3f87cb0cf8ec8196d7d801210385cabf57efd22267de723e80aac89c06136b058648ee5d4225746c695bc829d500000000
```

The initial transaction looks as follows: https://mempool.space/signet/tx/68e2d729431eae48ff8deda8b2437b80dfba1973d834ef25a7c9e84dcb772231

The caboose (the last output) has a script hash 2775ad76ba4cd805e2b41b9ef18c644904debd8fb19dbb50e7db061f85e2d5dd. One can [verify](https://emn178.github.io/online-tools/sha256.html) that the corresponding preimage is:
```
6a08000000000c000000
```
The "00000000" after "6a08" refers to the initial counter value 0. One can use a different initial input by replacing the P2WSH address correspondingly. This repository provides a tool, accessible through
"cargo run print_address", to calculate the signet address.

### Step 6: assemble the state update transaction using `test_consistency` 

An integration test `test_consistency` in `src/counter/mod.rs` allows one to prepare the transaction that can update the state by increasing the counter by 1.

One needs to first update `tx1_data` and `tx2_data` with the transaction hex (can be obtained from `./bitcoin-cli --datadir=signet gettransaction [txid]`). Here, 
`tx1_data` refers to the funding transaction, and `tx2_data` refers to the initialization transaction.

```rust
#[test]
fn test_consistency() {
    let tx1_data = "xxxxxx";
    let tx2_data = "xxxxxx";

    ...
}
```

Then one supplies the TxID.

```rust
let mut txid = [0u8; 32];
txid.copy_from_slice(&hex::decode("xxxxxx").unwrap());
txid.reverse();

let mut prev_txid = [0u8; 32];
prev_txid.copy_from_slice(&hex::decode("xxxxxx").unwrap());
prev_txid.reverse();
```

Here, the order is different. `txid` is the one for the initialization transaction, and `prev_txid` is the one for the funding transaction.

One can then run this test with `-- --nocapture`, it should provide the transaction hex, the randomizer being used, and the transaction size.

```
tx: 01000000000101312277cb4de8c9a725ef34d87319badf807b43b2a8ed8dff48ae1e4329d7e2680000000000ffffffff02920400000000000022512022d49e6696cb33db2d6b5d8c44b4dfd0c7e5c701bb8d4d23d9e81eca928310f14a0100000000000022002053cd9ef125f9dbe355c03bdffcb35dc5f5f7ab832cbc2e1d49277a2f18c555c70d08920400000000000022512022d49e6696cb33db2d6b5d8c44b4dfd0c7e5c701bb8d4d23d9e81eca928310f1010104af00000020312277cb4de8c9a725ef34d87319badf807b43b2a8ed8dff48ae1e4329d7e2680894110000000000002035a32cf60687b632706243b9ddc4b49f4b35344da604d90826ecca3ed07636ba1f78b6f879b8f0309a97cd6095f4898e1dfce07ab4e29d6cd6de04dbb274c6a424d4b0610c3363351b60993bc76d12fbb4afa68a132483c496444f1827053192750000000000040c000000fdbe0101004f040100000004000000007e7e7e748c7a825888748c7a82012288766b01227c7e7e6c7c084a010000000000007e026a08748c7a8b8c7600a069768c6b82539f630200007e6882549f6301007e6882549f6301007e68748c7a8254887e7ea8032200207c7e7ea87b7c7e6c7c527e748c7a82012088766b04000000007e7e748c7a825888766b7e527901227c7e7e04ffffffff7e6c7c6c7c748c7a82012088010004ffffffff7e7e7e20f40a48df4b2a70c8b4924bf2654661ed3d95fd66a313eb87237597c628e4a031767e7c7ea82079be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798766b766b767b7e7e207bb52d7a9fef58323eb1bf7a407db382d2f3f2d81bb1224f49fe518f6d48d37c767e7c7ea8748c7a82011f8876517e7b886c7c0202817e7e6cad0401000000748c7a82012488748c7a8200876b820124876c9b6982008763750500ffffffff7e517c7e676b0500ffffffff766c7c7e7e7e527c7e687e527e527a7e537a01227c7e7e084a010000000000007e026a08537a82539f630200007e6882549f6301007e6882549f6301007e68748c7a8254887e7ea8032200207c7e7e04000000007ea8a88721c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac000000000
randomizer: 175
tx length: 824
```

One can then configure information such as the fee. In the example, we have overpaid the fee (3000 sats), while in fact about 210 sats would be sufficient.
```rust
let prev_counter = 0;
let prev_randomizer = 12;
let prev_balance = 4500;
let new_balance = 4500 - DUST_AMOUNT - 3000;
```

One can then submit this transaction using `./bitcoin-cli --datadir=signet sendrawtransaction`. It is important to use the Taproot Wizards's 
relay node here to make sure that the transaction can be routed into the miner without being ignored by other relay nodes.

### Step 7: observe the new state

One can find the example transaction here: https://mempool.space/signet/tx/b01aa4eaba04b0f220e2dd6f57a6331e2e14c64d14d62ddf99a791739b9d0d41

The new caboose has a hash 53cd9ef125f9dbe355c03bdffcb35dc5f5f7ab832cbc2e1d49277a2f18c555c7. One can verify that its preimage is:
```
6a0801000000af000000
```
The "01000000" after "6a08" refers to the new counter value 1. The following "af000000" refers to the randomizer. 


