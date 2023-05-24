mod edit_distance;
mod filter;
mod hamming_distance;
mod trie;
mod base58;

fn main() {
    // Test 编辑距离——汉明距离
    hamming_distance::it_work();

    // Test 编辑距离
    edit_distance::it_work();

    // Test Trie
    trie::it_work();

    // Test 过滤器
    filter::it_work();

    // Test Base58
    base58::it_work();
}
