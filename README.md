## covenants-gadgets

This repository implements Bitcoin script gadgets that make it easy for developers to build applications from Bitcoin 
script. In particular, it implements two tools for reflection in Bitcoin.
- [CAT and Schnorr tricks](https://medium.com/blockstream/cat-and-schnorr-tricks-i-faf1b59bd298) from Andrew Poelstra. 
Using the Schnorr signature scheme with a dummy public key and a dummy random element R (both are equal to the secp256k1 group 
generator), one can repurpose the Schnorr signature verification to get a hash of the key information in the transaction, which enables 
the input to perform a reflection on the transaction that is going to spend it. In short, it reflects on the current transaction.
- Reflection on the [txid](https://en.bitcoin.it/wiki/Transaction) (not wtxid) allows one to check key information about the transaction where this input was an output 
of that previous transaction. This is done by simply reconstructing a transaction without any segregated witness stuffs 
and compute its double SHA256. It relies on the CAT and Schnorr tricks, since it needs to obtain the outpoint information of the 
present transaction. In short, it reflects on the previous transaction.

These two tools allow one to build covenants. 

A minimalistic example, which implements a counter, is showcased in [Bitcoin-Wildlife-Sanctuary/covenants-examples](https://github.com/Bitcoin-Wildlife-Sanctuary/covenants-examples). It enforces that the counter can only be increased by one from the previous transaction each time.

Now we provide some background on the two tools discussed above.

### CAT and Schnorr tricks

In P2TR, `OP_CHECKSIGVERIFY` accepts the top stack element as the public key, and the second-to-top stack element as the 
signature. The public key is a compressed secp256k1 public key, and the signature is the Schnorr signature with an (optional) 
signature hash type byte.

The signing algorithm works as follows, with $x$ as the secret key and $Y = x\cdot G$ as the public key. 
1. Sample a random scalar field element: $k$.
2. Compute $R = k\cdot G$.
3. Compute $e = H(R || Y || m)$ where $m$ is the message.
4. Compute $s = k + xe$.

In the landmark article from Andrew Poelstra from Blockstream, it shows that if we use the secp256k1 group generator as 
the public key (meaning that $x = 1$) and the random point $R$ (meaning that $k = 1$), we can have $s = 1 + e$ where $e$ 
is almost like a hash of the message that can be recomputed inside the Bitcoin script using `OP_SHA256` and other opcodes
including `OP_CAT`. 

One may wonder why not pick $k = 0$ as it could give a cleaner expression $s = e$. This is indeed infeasible because in 
that case, the signature would be rejected due to $R$ being invalid.

There are many ways to handle the "+1" part. The article provides an idea: by tweaking the transaction without changing 
its utility, one can get different $e$, and with a possibility of about $1/256$, one would hit a hash that ends with 0x01. 
Replacing the last byte 0x01 with 0x02 gives the "+1" result, through some Bitcoin script that uses a hint.

Variants exist in practice. Taproot Wizards's [vault](https://github.com/taproot-wizards/purrfect_vault/) searches for 
$e$ ending at 0x00 and appending 0x01 on it, and it uses the sequence field to tweak the transaction. 
The [covenants-examples](https://github.com/Bitcoin-Wildlife-Sanctuary/covenants-examples) repository uses a more tedious 
approach, by making the caboose tweakable. In fact, it is possible to use many other numbers, not restricted to 0x01 or 
0x00. We just want to use a Bitcoin integer that only takes zero or one byte to represent. Nevertheless, the process of 
doing such tweaking, called "grinding", is generally very efficient.

The message in the hashing for $e$ is a hash of key elements in the script, including information about the input, the output, 
and input's outpoints and balances. One can find more detail in [BIP-341 "Taproot: SegWit version 1 spending rules"](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki),
copy-pasted as follows. This is in general sufficient to build useful self reflection.

--------------

- Control:
  * ''hash_type'' (1).
- Transaction data:
  * ''nVersion'' (4): the ''nVersion'' of the transaction.
  * ''nLockTime'' (4): the ''nLockTime'' of the transaction.
  * If the ''hash_type & 0x80'' does not equal <code>SIGHASH_ANYONECANPAY</code>:
    - ''sha_prevouts'' (32): the SHA256 of the serialization of all input outpoints.
    - ''sha_amounts'' (32): the SHA256 of the serialization of all input amounts.
    - ''sha_scriptpubkeys'' (32): the SHA256 of all spent outputs' ''scriptPubKeys'', serialized as script inside <code>CTxOut</code>.
    - ''sha_sequences'' (32): the SHA256 of the serialization of all input ''nSequence''.
  * If ''hash_type & 3'' does not equal <code>SIGHASH_NONE</code> or <code>SIGHASH_SINGLE</code>:
    - ''sha_outputs'' (32): the SHA256 of the serialization of all outputs in <code>CTxOut</code> format.
- Data about this input:
  * ''spend_type'' (1): equal to ''(ext_flag * 2) + annex_present'', where ''annex_present'' is 0 if no annex is present, or 1 otherwise (the original witness stack has two or more witness elements, and the first byte of the last element is ''0x50'')
  * If ''hash_type & 0x80'' equals <code>SIGHASH_ANYONECANPAY</code>:
    - ''outpoint'' (36): the <code>COutPoint</code> of this input (32-byte hash + 4-byte little-endian).
    - ''amount'' (8): value of the previous output spent by this input.
    - ''scriptPubKey'' (35): ''scriptPubKey'' of the previous output spent by this input, serialized as script inside <code>CTxOut</code>. Its size is always 35 bytes.
    - ''nSequence'' (4): ''nSequence'' of this input.
  * If ''hash_type & 0x80'' does not equal <code>SIGHASH_ANYONECANPAY</code>:
    - ''input_index'' (4): index of this input in the transaction input vector. Index of the first input is 0.
  * If an annex is present (the lowest bit of ''spend_type'' is set):
    - ''sha_annex'' (32): the SHA256 of ''(compact_size(size of annex) || annex)'', where ''annex'' includes the mandatory ''0x50'' prefix.
- Data about this output:
  * If ''hash_type & 3'' equals <code>SIGHASH_SINGLE</code>:
    - ''sha_single_output'' (32): the SHA256 of the corresponding output in <code>CTxOut</code> format.

--------------

### Txid reflection

Txid is the double hash of everything in the transaction that is not a segregated witness stuff. To be precise, it includes 
the following information:
- transaction version number
- number of inputs
- serialized inputs
- number of outputs
- serialized outputs

Each serialized input includes the following information:
- outpoint
- script signature (empty for P2WPKH, P2WSH, P2TR)
- sequence number

Note that the script and witness elements are "segregated" and therefore they do not affect the txid. 

Each serialized output includes the following information:
- amount
- script public key

We use txid reflection to obtain the "previous state". This is done by putting the previous state information in one of 
the serialized output as a P2WSH UTXO. Since we are able to use reflection to find out the script public key, we are able 
to verify the information in that UTXO.

### Overview

Developers can start from the `wizards` folder to find out the Bitcoin script gadgets that suit their needs. There are 
six wizards, some of which are smaller wizards under bigger wizards.
- [tag_csv_preimage.rs](https://github.com/Bitcoin-Wildlife-Sanctuary/covenants-gadgets/blob/main/src/wizards/tap_csv_preimage.rs): 
a wizard that rebuilds the preimage for the taproot CheckSigVerify.
- [tx.rs](https://github.com/Bitcoin-Wildlife-Sanctuary/covenants-gadgets/blob/main/src/wizards/tx.rs): a wizard that rebuilds 
the preimage for calculating the txid.

The smaller wizards are as follows.
- [ext.rs](https://github.com/Bitcoin-Wildlife-Sanctuary/covenants-gadgets/blob/main/src/wizards/ext.rs): a smaller wizard 
that rebuilds the extension in the taproot CheckSigVerify. 
- [outpoint.rs](https://github.com/Bitcoin-Wildlife-Sanctuary/covenants-gadgets/blob/main/src/wizards/outpoint.rs): a smaller 
wizard that constructs the outpoint structure of an input.
- [tx_in.rs](https://github.com/Bitcoin-Wildlife-Sanctuary/covenants-gadgets/blob/main/src/wizards/tx_in.rs): a smaller 
wizard that reconstructs the transaction input for segwit UTXOs, without the witness.
- [tx_out.rs](https://github.com/Bitcoin-Wildlife-Sanctuary/covenants-gadgets/blob/main/src/wizards/tx_out.rs): a smaller 
wizard that reconstructs the transaction output.

<p align="center">
  <img src="fig/young-rijndael.png" />
</p>

The rest of the repository consists of the building blocks that these wizards use to construct each of the field. It can 
be most helpful to check out the Rust documentation below.

<p align="center">
  <a href="https://bitcoin-wildlife-sanctuary.github.io/covenants-gadgets/">
  https://bitcoin-wildlife-sanctuary.github.io/covenants-gadgets/
  </a>
</p>

### Acknowledgment

A portion of the code is contributed by L2 Iterative (L2IV), a crypto VC based in San Francisco and Hong Kong. The work 
receives support from Starkware, who is a limited partner in L2IV. For disclosure, L2IV has also invested into numerous 
companies active in the Bitcoin ecosystem, but this work is open-source and nonprofit, and is not intended for competition. 
The code is not investment advice.

There are also community members contributing to the code and contributing to the ideas. Bitcoin Wildlife Sanctuary is a 
public-good project supported by many people including from Taproot Wizards.