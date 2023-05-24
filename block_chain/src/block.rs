use crate::serializer::{hash_str, sericalize};
use chrono::prelude::*;
use serde::Serialize;
use std::thread;
use std::time::Duration;

// 区块头
#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    pub time: i64,
    pub pre_hash: String,
    pub txs_hash: String,
}

// 区块
#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub tranxs: String,
    pub hash: String,
}

impl Block {
    pub fn new(txs: String, pre_hash: String) -> Self {
        // 延迟3秒模拟挖矿
        println!("Start mining...");
        thread::sleep(Duration::from_secs(3));

        // 准备时间、计算交易哈希值
        let time = Utc::now().timestamp();
        let txs_ser = sericalize(&txs);
        let txs_hash = hash_str(&txs_ser);
        let mut block = Block {
            header: BlockHeader {
                time,
                pre_hash,
                txs_hash,
            },
            tranxs: txs,
            hash: "".to_string(),
        };
        block.set_hash();
        println!("Produce a new block!\n");
        block
    }

    // 计算并设置区块哈希值
    pub fn set_hash(&mut self) {
        let header = sericalize(&(self.header));
        self.hash = hash_str(&header);
    }
}
