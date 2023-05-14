mod binary;
mod hash;
mod sequential;

fn main() {
    // Test 顺序查找
    sequential::it_work();

    // Test 二分查找
    binary::it_work();

    // Test 哈希查找
    hash::it_work();
}
