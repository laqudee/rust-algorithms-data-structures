/// Rust实现 递归二分法

pub fn search(nums: &[i32], num: i32) -> bool {
    // 基本情况1: 项不存在
    if 0 == nums.len() {
        return false;
    }

    let mid: usize = nums.len() >> 1;

    // 基本情况2: 项存在
    if num == nums[mid] {
        return true;
    } else if num < nums[mid] {
        // 减少问题规模
        return search(&nums[..mid], num);
    } else {
        return search(&nums[mid + 1..], num);
    }
}
