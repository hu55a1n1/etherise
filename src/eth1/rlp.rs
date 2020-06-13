use ethereum_types::{Address, Bloom, H256, U256};
use keccak_hash::keccak;
use rlp::{DecoderError, Rlp};
use serde_json::{json, Value};

pub struct EthRlp<'a>(Rlp<'a>);

impl EthRlp<'_> {
    pub fn new(bytes: &[u8]) -> EthRlp {
        EthRlp(Rlp::new(bytes))
    }
}
