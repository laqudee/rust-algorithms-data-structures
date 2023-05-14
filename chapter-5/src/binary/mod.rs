/// Rust 实现二分查找

pub fn binary_search(nums: &[i32], num: i32) -> bool {
    let mut low = 0;
    let mut high = nums.len() - 1;
    let mut found = false;

    // 注意是 <= 不是 <
    while low <= high && !found {
        let mid: usize = (low + high) >> 1;

        // 若low + high 可能溢出，可转换为剑法
        // let mid: usize = low + ((high - low) >> 1);

        if num == nums[mid] {
            found = true;
        } else if num < nums[mid] {
            high = mid - 1; // num < 中间值，省去后半部数据
        } else {
            low = mid + 1; // num >= 中间值，省去前半部分数据
        }
    }
    found
}

pub fn it_work() {
    let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 3;
    let found = binary_search(&nums, target);
    println!("{target} is in nums: {}", found);
}
