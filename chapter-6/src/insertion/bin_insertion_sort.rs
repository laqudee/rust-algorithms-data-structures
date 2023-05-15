pub fn sort(nums: &mut [i32]) {
    let mut temp;
    let mut left;
    let mut mid;
    let mut right;

    for i in 1..nums.len() {
        left = 0; // 已排序数组左右边界
        right = i - 1;
        temp = nums[i]; // 带排序数据

        // 二分法找到temp位置
        while left <= right {
            mid = (left + right) >> 1;
            if temp < nums[mid] {
                // 防止出现right = 0 - 1
                if 0 == mid {
                    break;
                }
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        // 将数据后移，留出空位
        for j in (left..=i - 1).rev() {
            nums.swap(j, j + 1);
        }

        //将temp 插入空位
        if left != i {
            nums[left] = temp;
        }
    }
}
