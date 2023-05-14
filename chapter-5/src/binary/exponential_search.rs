use super::binary_search;

/// Rust 实现指数查找
/// 适合已排序且无边界的数据

pub fn search(nums: &[i32], target: i32) -> bool {
    let size = nums.len();
    if size == 0 {
        return false;
    };

    // 逐步找到上界
    let mut high = 1usize;
    while high < size && nums[high] < target {
        high <<= 1;
    }

    // 上界的一半一定可以作为下界
    let low = high >> 1;

    // 使用前面实现的二分查找
    binary_search(&nums[low..size.min(high + 1)], target)
}
