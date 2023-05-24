use crate::pow::ProofOfWork;
use crate::serializer::{hash_str, sericalize};
use chrono::prelude::*;
use serde::Serialize;
use std::thread;
use std::time::Duration;

// 区块头
#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    pub nonce: u32,
    pub bits: u32,
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
    pub fn new(txs: String, pre_hash: String, bits: u32) -> Self {
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
                bits,
                nonce: 0,
            },
            tranxs: txs,
            hash: "".to_string(),
        };

        // 初始化挖矿任务并开始挖矿
        let pow = ProofOfWork::new(bits);
        pow.run(&mut block);

        block
    }
}
