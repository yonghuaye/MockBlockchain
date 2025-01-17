//Hash TimeStamp
use chrono::prelude::*;
use utils::coder;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String, //交易的默克尔哈希
    pub pre_hash: String,
}
#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String, //区块头哈希
    pub data: String //交易数据
}

impl Block {
    //区块哈希
    fn set_hash(&mut self) {
        let header = coder::my_serialize(&(self.header));
        self.hash = coder::get_hash(&header[..]);
    }

    pub fn new_block(data: String, pre_hash: String) -> Block {
        let transaction = coder::my_serialize(&data);
        let tx_hash = coder::get_hash(&transaction[..]);
        // let mut tx_hash: [u8; 32] = [0; 32];
        // coder::get_hash(&transaction[..], &mut tx_hash);

        let time = Utc::now().timestamp();

        let mut block = Block {
            header: BlockHeader {
                time: time,
                tx_hash: tx_hash,  //transactions data merkle root hash
                // tx_hash: str::from_utf8(&tx_hash).unwrap().to_string(),  //transactions data merkle root hash
                // tx_hash: tx_hash.iter().map(|&c| c as char).collect::<String>(),//交易的默克尔哈希
                pre_hash: pre_hash,
                // bits: bits,
                // nonce: 0,
            },
            hash: "".to_string(),
            data: data,
        };
        block.set_hash();
        block
    }
}