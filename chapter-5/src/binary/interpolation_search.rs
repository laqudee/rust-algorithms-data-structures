/// Rust 实现内插查找算法
/// 通过差值算法找到待查值的上下界
/// 
/// 原书算法有问题

pub fn search(nums: &[i32], target: i32) -> bool {
    if nums.is_empty() {
        return false;
    }
    let mut low = 0usize;
    let mut high = nums.len() - 1;
    loop {
        let low_val = nums[low];
        let high_val = nums[high];

        if high <= low || target < low_val || target > high_val {
            break;
        }

        // 计算插值位置
        let offset = ((target - low_val) * (high - low) as i32) / (high_val - low_val);
        let interpolant = low + offset as usize;

        println!("low: {low}, high: {high}, offset: {offset}, interpolant: {interpolant}");

        // 更新上下界high、low
        if nums[interpolant] > target {
            high = interpolant - 1;
        } else if nums[interpolant] < target {
            low = interpolant + 1;
        } else {
            break;
        }
    }

    // 判断最终确定的下界是否是target
    if target == nums[low] {
        true
    } else {
        false
    }
}
