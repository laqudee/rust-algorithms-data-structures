/// Rust实现快速排序

fn quick_sort(nums: &mut [i32], low: usize, high: usize) {
    if low < high {
        let split = partition(nums, low, high);
        if split > 1 {
            // 防止越界 （split <= 1） 和语法错误
            quick_sort(nums, low, split - 1);
        }
        quick_sort(nums, split + 1, high);
    }
}

fn partition(nums: &mut [i32], low: usize, high: usize) -> usize {
    let mut lm = low; // 左标记
    let mut rm = high; // 右标记
    loop {
        // 左标记不断右移
        while lm < rm && nums[lm] <= nums[low] {
            lm += 1;
        }
        // 右标记不断左移
        while lm <= rm && nums[rm] >= nums[low] {
            rm -= 1;
        }

        // 左标记越过🈶️标记时退出并交换左右标记数据
        if lm > rm {
            break;
        } else {
            nums.swap(lm, rm);
        }
    }
    nums.swap(low, rm);

    rm
}

pub fn it_work() {
    let mut nums = [54, 23, 83, 17, 77, 31, 44, 55, 20];
    let len = nums.len();
    quick_sort(&mut nums, 0, (len - 1) as usize);
    println!("quick sort -- sorted nums: {:?}", nums);
}
