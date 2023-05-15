/// Rust 实现插入排序
pub mod bin_insertion_sort;

fn sort(nums: &mut [i32]) {
    for i in 1..nums.len() {
        let mut pos = i;
        let curr = nums[i];

        while pos > 0 && curr < nums[pos - 1] {
            nums[pos] = nums[pos - 1]; // 向后移动数据
            pos -= 1;
        }
        nums[pos] = curr; // 插入数据
    }
}
pub fn it_work() {
    let mut nums = [54, 23, 83, 17, 77, 31, 44, 55, 20];
    sort(&mut nums);
    println!("insertion -- sorted nums: {:?}", nums);

    // Test 二分法插入排序
    let mut nums = [1, 3, 2, 8, 6, 4, 9, 7, 5, 10];
    bin_insertion_sort::sort(&mut nums);
    println!("bin_insertion_sort -- sorted nums: {:?}", nums);
}
