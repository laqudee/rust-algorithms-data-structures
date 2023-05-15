/// 梳排序
/// 开始比较间距设定为数组长度，并在循环中以固定的比率递减，通常递减率为1.3

pub fn sort(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }

    let mut i;
    let mut gap: usize = nums.len();

    // 大致排序，数据基本有序
    while gap > 0 {
        gap = (gap as f32 * 0.8) as usize;
        i = gap;
        while i < nums.len() {
            if nums[i - gap] > nums[i] {
                nums.swap(i - gap, i);
            }
            i += 1;
        }
    }

    // 细致调节部分无序数据，exchange 控制是否继续交换数据
    let mut _exchange = true;
    while gap > 0 {
        _exchange = false;
        i = 0;
        while i < nums.len() - 1 {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
                _exchange = true;
            }
            i += 1;
        }
    }
}
