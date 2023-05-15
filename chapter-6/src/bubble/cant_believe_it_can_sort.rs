/// - 一种2021年发表的不需要出来边界下标值的排序算法
///  - 类似插入排序
///  - 看起来降序排序，实际是升序排序

pub fn cbic_sort1(nums: &mut [i32]) {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if nums[i] < nums[j] {
                nums.swap(i, j);
            }
        }
    }
}

// 对上述方法进行优化
pub fn cbic_sort2(nums: &mut [i32]) {
    if nums.len() < 2 {
        return;
    }

    for i in 1..nums.len() {
        for j in 0..i {
            if nums[i] < nums[j] {
                nums.swap(i, j);
            }
        }
    }
}
