use ethereum_types::{Address, Bloom, H256, U256};
use keccak_hash::keccak;
use rlp::{DecoderError, Rlp};

pub type Bytes = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Create,
    Call(Address),
}

impl Default for Action {
    fn default() -> Action { Action::Create }
}

impl rlp::Decodable for Action {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        if rlp.is_empty() {
            if rlp.is_data() {
                Ok(Action::Create)
            } else {
                Err(DecoderError::RlpExpectedToBeData)
            }
        } else {
            Ok(Action::Call(rlp.as_val()?))
        }
    }
}


#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub nonce: U256,
    pub gas_price: U256,
    pub gas: U256,
    pub action: Action,
    pub value: U256,
    pub data: Bytes,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnverifiedTransaction {
    unsigned: Transaction,
    v: u64,
    r: U256,
    s: U256,
    hash: H256,
}

impl rlp::Decodable for UnverifiedTransaction {
    fn decode(d: &Rlp) -> Result<Self, DecoderError> {
        if d.item_count()? != 9 {
            return Err(DecoderError::RlpIncorrectListLen);
        }
        let hash = keccak(d.as_raw());
        Ok(UnverifiedTransaction {
            unsigned: Transaction {
                nonce: d.val_at(0)?,
                gas_price: d.val_at(1)?,
                gas: d.val_at(2)?,
                action: d.val_at(3)?,
                value: d.val_at(4)?,
                data: d.val_at(5)?,
            },
            v: d.val_at(6)?,
            r: d.val_at(7)?,
            s: d.val_at(8)?,
            hash,
        })
    }
}


#[derive(Debug, Clone)]
pub struct Header {
    parent_hash: H256,
    timestamp: u64,
    number: U256,
    author: Address,
    transactions_root: H256,
    uncles_hash: H256,
    extra_data: Bytes,
    state_root: H256,
    receipts_root: H256,
    log_bloom: Bloom,
    gas_used: U256,
    gas_limit: U256,
    difficulty: U256,
    seal: Vec<Bytes>,
    hash: Option<H256>,
}

impl rlp::Decodable for Header {
    fn decode(r: &Rlp) -> Result<Self, DecoderError> {
        let mut blockheader = Header {
            parent_hash: r.val_at(0)?,
            uncles_hash: r.val_at(1)?,
            author: r.val_at(2)?,
            state_root: r.val_at(3)?,
            transactions_root: r.val_at(4)?,
            receipts_root: r.val_at(5)?,
            log_bloom: r.val_at(6)?,
            difficulty: r.val_at(7)?,
            number: r.val_at(8)?,
            gas_limit: r.val_at(9)?,
            gas_used: r.val_at(10)?,
            timestamp: r.val_at(11)?,
            extra_data: r.val_at(12)?,
            seal: vec![],
            hash: keccak(r.as_raw()).into(),
        };

        for i in 13..r.item_count()? {
            blockheader.seal.push(r.at(i)?.as_raw().to_vec().into())
        }

        Ok(blockheader)
    }
}

#[cfg(test)]
mod tests {
    use rustc_hex::FromHex;

    use super::*;

    #[test]
    fn test_header_seal_fields() {
        let header_rlp: Bytes = "f901f9a0d405da4e66f1445d455195229624e133f5baafe72b5cf7b3c36c12c8146e98b7a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347948888f1f195afa192cfee860698584c030f4c9db1a05fb2b4bfdef7b314451cb138a534d225c922fc0e5fbe25e451142732c3e25c25a088d2ec6b9860aae1a2c3b299f72b6a5d70d7f7ba4722c78f2c49ba96273c2158a007c6fdfa8eea7e86b81f5b0fc0f78f90cc19f4aa60d323151e0cac660199e9a1b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008302008003832fefba82524d84568e932a80a0a0349d8c3df71f1a48a9df7d03fd5f14aeee7d91332c009ecaff0a71ead405bd88ab4e252a7e8c2a23".from_hex().unwrap();
        let mix_hash: Bytes = "a0a0349d8c3df71f1a48a9df7d03fd5f14aeee7d91332c009ecaff0a71ead405bd".from_hex().unwrap();
        let nonce: Bytes = "88ab4e252a7e8c2a23".from_hex().unwrap();

        let header: Header = rlp::decode(&header_rlp).expect("error decoding header");
        let seal_fields = header.seal.clone();
        assert_eq!(seal_fields.len(), 2);
        assert_eq!(seal_fields[0], mix_hash);
        assert_eq!(seal_fields[1], nonce);
    }

    #[test]
    fn decode_and_encode_header() {
        let header_rlp: Bytes = "f901f9a0d405da4e66f1445d455195229624e133f5baafe72b5cf7b3c36c12c8146e98b7a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347948888f1f195afa192cfee860698584c030f4c9db1a05fb2b4bfdef7b314451cb138a534d225c922fc0e5fbe25e451142732c3e25c25a088d2ec6b9860aae1a2c3b299f72b6a5d70d7f7ba4722c78f2c49ba96273c2158a007c6fdfa8eea7e86b81f5b0fc0f78f90cc19f4aa60d323151e0cac660199e9a1b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008302008003832fefba82524d84568e932a80a0a0349d8c3df71f1a48a9df7d03fd5f14aeee7d91332c009ecaff0a71ead405bd88ab4e252a7e8c2a23".from_hex().unwrap();
        let header: Header = rlp::decode(&header_rlp).expect("error decoding header");
        assert_eq!(header.seal, vec![Bytes::from(vec![160, 160, 52, 157, 140, 61, 247, 31, 26,
                                                      72, 169, 223, 125, 3, 253, 95, 20, 174, 238,
                                                      125, 145, 51, 44, 0, 158, 202, 255, 10, 113,
                                                      234, 212, 5, 189]),
                                     Bytes::from(vec![136, 171, 78, 37, 42, 126, 140, 42, 35])]);
    }

    #[test]
    fn reject_header_with_large_timestamp() {
        // The encoding contains a large timestamp (295147905179352825856)
        let header_rlp: Bytes = "f901f9a0d405da4e66f1445d455195229624e133f5baafe72b5cf7b3c36c12c8146e98b7a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347948888f1f195afa192cfee860698584c030f4c9db1a05fb2b4bfdef7b314451cb138a534d225c922fc0e5fbe25e451142732c3e25c25a088d2ec6b9860aae1a2c3b299f72b6a5d70d7f7ba4722c78f2c49ba96273c2158a007c6fdfa8eea7e86b81f5b0fc0f78f90cc19f4aa60d323151e0cac660199e9a1b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008302008003832fefba82524d891000000000000000000080a0a0349d8c3df71f1a48a9df7d03fd5f14aeee7d91332c009ecaff0a71ead405bd88ab4e252a7e8c2a23".from_hex().unwrap();

        // This should fail decoding timestamp
        let header: Result<Header, _> = rlp::decode(&header_rlp);
        assert_eq!(header.unwrap_err(), rlp::DecoderError::RlpIsTooBig);
    }
}
