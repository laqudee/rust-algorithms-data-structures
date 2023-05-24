use bincode;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use serde::Serialize;

// 序列化数据
pub fn sericalize<T: ?Sized>(value: &T) -> Vec<u8>
where
    T: Serialize,
{
    bincode::serialize(value).unwrap()
}

// 计算value哈希值并以String形式返回
pub fn hash_str(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}
