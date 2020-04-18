use bls::PublicKeyBytes;
use serde::{Deserialize, Serialize};
use ssz_new_derive::{SszDecode, SszEncode};
use ssz_types::{BitList, FixedVector, VariableList};
use tree_hash::TreeHash;
use tree_hash_derive::TreeHash;
use typenum::{Sum, U1};

pub use ethereum_types::H256;

use crate::{config::Config, primitives::*};
use crate::custody_game_types::{CustodyKeyReveal, EarlyDerivedSecretReveal, CustodySlashing};

pub type Root = H256;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash)]
pub struct ShardTransition<C: Config>{
    pub start_slot: Slot,
    pub shard_block_lengths: VariableList<u64, C::MaxShardBlocksPerAttestation>,
    pub shard_data_roots: VariableList<H256, C::MaxShardBlocksPerAttestation>,
    pub shard_states: VariableList<ShardState, C::MaxShardBlocksPerAttestation>,
    pub proposer_signature_aggregate: SignatureBytes
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash)]
pub struct ShardState{
    slot: Slot,
    gasprice: Gwei,
    data: H256,
    latest_block_root: Root
}