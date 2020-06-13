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
            Ok(9) => self.to_json_tx(),
            Ok(14..=16) => self.to_json_blockheader(),
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
    fn to_json_tx(&self) -> Result<Value, DecoderError> {
        let mut tx = json!({
            "nonce": self.0.val_at::<U256>(0)?,
            "gas_price": self.0.val_at::<U256>(1)?,
            "gas": self.0.val_at::<U256>(2)?,
            "value": self.0.val_at::<U256>(4)?,
            "data": self.0.val_at::<Vec<u8>>(5)?.hex_display().to_string(),
            "v": self.0.val_at::<u64>(6)?,
            "r": self.0.val_at::<U256>(7)?,
            "s": self.0.val_at::<U256>(8)?,
        });
        let rlp_addr = self.0.at(3)?;
        if rlp_addr.is_empty() {
            if !rlp_addr.is_data() {
                return Err(DecoderError::RlpExpectedToBeData);
            }
            // else => create tx
        } else {
            tx["address"] = json!(self.0.val_at::<Address>(3)?);
        }
        Ok(tx)
    }

    fn to_json_blockheader(&self) -> Result<Value, DecoderError> {
        let mut blockheader = json!({
            "parent_hash": self.0.val_at::<H256>(0)?,
            "uncles_hash": self.0.val_at::<H256>(1)?,
            "author": self.0.val_at::<Address>(2)?,
            "state_root": self.0.val_at::<H256>(3)?,
            "transactions_root": self.0.val_at::<H256>(4)?,
            "receipts_root": self.0.val_at::<H256>(5)?,
            "log_bloom": self.0.val_at::<Bloom>(6)?,
            "difficulty": self.0.val_at::<U256>(7)?,
            "number": self.0.val_at::<U256>(8)?,
            "gas_limit": self.0.val_at::<U256>(9)?,
            "gas_used": self.0.val_at::<U256>(10)?,
            "timestamp": self.0.val_at::<u64>(11)?,
            "extra_data": self.0.val_at::<Vec<u8>>(12)?.hex_display().to_string(),
            "hash": keccak(self.0.as_raw()),
        });

        let mut seal = vec![];
        for i in 13..self.0.item_count()? {
            seal.push(self.0.val_at::<Vec<u8>>(i)?.hex_display().to_string());
        }
        blockheader["seal"] = json!(seal);
        Ok(blockheader)
    }

}
