# 区块链

## 区块链及比特币原理

- 区块链技术是利用链式数据结构来验证与存储数据、利用分布式节点共识算法来生成和更新数据、利用密码学来保证数据传输和访问安全、利用自动化脚本的智能合约来编程和操作数据的一种全新的分布式基础架构和计算范式

- 简单来说，区块链就是去中心化的分布式账本

- 区块链是一个分布式的交易系统：
  - 区块
  - 区块链
  - 交易
  - 账户
  - 旷工
  - 手续费
  - 奖励

## 基础区块链

- 区块：
  - 区块头，包含前一个区块的哈希pre_hash，当前区块交易哈希tx_hash、区块打包时间time
  - 区块体，包含所有交易transactions
  - 区块哈希，是计算区块头和区块体得到的哈希值

- 哈希值非常重要，首先来实现哈希计算，计算之前先将区块结构体序列化，再计算哈希值

```rs
pub struct Block {
  pub header: BlockHeader,
  pub tranxs: String,
  pub hash: String,
}

pub struct BlockHeader {
  pub time: i64,
  pub pre_hash: String,
  pub txs_hash: String,
}
```