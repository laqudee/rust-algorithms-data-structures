/// 尾递归优化
/// 将运算结果传入递归函数，建设栈占用

pub fn nums_sum3(sum: i32, nums: &[i32]) -> i32 {
    if 1 == nums.len() {
        sum + nums[0]
    } else {
        nums_sum3(sum + nums[0], &nums[1..])
    }
}

pub fn nums_sum4(sum: i32, nums: &[i32]) -> i32 {
    if 1 == nums.len() {
        sum + nums[0]
    } else {
        nums_sum4(sum + nums[nums.len() - 1], &nums[..nums.len() - 1])
    }
}
