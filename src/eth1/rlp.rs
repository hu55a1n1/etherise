use ethereum_types::{Address, Bloom, H256, U256};
use keccak_hash::keccak;
use rlp::{DecoderError, Rlp};
use serde_json::{json, Value};

use crate::eth1::{HexDisplayExt};

pub struct EthRlp<'a>(Rlp<'a>);

impl EthRlp<'_> {
    pub fn new(bytes: &[u8]) -> EthRlp {
        EthRlp(Rlp::new(bytes))
    }

    pub fn to_json(&self) -> Result<Value, DecoderError> {
        match self.0.item_count() {
            Ok(3) => self.to_json_log(),
            _ => Err(DecoderError::Custom("unknown RLP data"))
        }
    }

    fn to_json_log(&self) -> Result<Value, DecoderError> {
        let mut log = json!({
            "address": self.0.val_at::<Address>(0)?,
            "data": self.0.val_at::<Vec<u8>>(2)?.hex_display().to_string(),
        });

        let rlp_topics = self.0.at(1)?;
        let mut topics = vec![];
        for i in 0..rlp_topics.item_count()? {
            topics.push(rlp_topics.val_at::<H256>(i)?);
        }
        log["topics"] = json!(topics);
        Ok(log)
    }
}
