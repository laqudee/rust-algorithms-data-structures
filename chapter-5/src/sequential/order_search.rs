/// 用在已排序数据集上的顺序查找算法，通过设置stop变量来控制查找超出范围时立即停止查找以节约时间

pub fn order_search(nums: &[i32], num: i32) -> bool {
    let mut pos: usize = 0;
    let mut found = false;
    let mut stop = false; // 控制遇到有序数据时退出

    while pos < nums.len() && !found && !stop {
        if num == nums[pos] {
            found = true;
        } else if num < nums[pos] {
            stop = true; // 数据有序,查找超出范围，退出
        } else {
            pos += 1;
        }
    }
    found
}
