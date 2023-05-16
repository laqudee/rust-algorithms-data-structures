/// 希尔排序

fn shell_sort(nums: &mut [i32]) {
    // 插入排序函数（内部），数据相隔距离为gap
    fn ist_sort(nums: &mut [i32], start: usize, gap: usize) {
        let mut i = start + gap;

        while i < nums.len() {
            let mut pos = i;
            let curr = nums[pos];
            while pos >= gap && curr < nums[pos - gap] {
                nums[pos] = nums[pos - gap];
                pos -= gap;
            }

            nums[pos] = curr;
            i += gap;
        }
    }

    // 每次让gap减少一半直到1
    let mut gap = nums.len() / 2;
    while gap > 0 {
        for start in 0..gap {
            ist_sort(nums, start, gap);
        }
        gap /= 2;
    }
}

pub fn it_work() {
    let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];
    shell_sort(&mut nums);
    println!("shell -- sorted nums: {:?}", nums);
}
