/// 选择排序

fn selection_sort(nums: &mut Vec<i32>) {
    let mut left = nums.len() - 1; // 待排序数据下标
    while left > 0 {
        let mut pos_max = 0;
        for i in 1..=left {
            if nums[i] > nums[pos_max] {
                pos_max = i; // 选择当前轮次最大值下标
            }
        }
        // println!("left: {}; pos_max: {}", left, pos_max);
        // 数据交换，完成一个数据的排序，待排序数据量减少1
        nums.swap(left, pos_max);
        left -= 1;
    }
}

pub fn it_work() {
    let mut nums = vec![54, 32, 99, 18, 75, 31, 43, 56, 21, 22];
    selection_sort(&mut nums);
    println!("selection -- sorted nums: {:?}", nums);
}
