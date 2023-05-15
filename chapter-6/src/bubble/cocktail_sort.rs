/// 鸡尾酒排序（冒泡排序的变种）
/// 双端排序

pub fn sort(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }

    // bubble 控制是否继续冒泡
    let mut bubble = true;
    let len = nums.len();
    for i in 0..(len >> 1) {
        if bubble {
            bubble = false;

            // 从左到右冒泡
            for j in i..(len - i - 1) {
                if nums[j] > nums[j + 1] {
                    nums.swap(j, j + 1);
                    bubble = true;
                }
            }

            // 从右往左冒泡
            for j in (i + 1..=(len - i - 1)).rev() {
                if nums[j] < nums[j - 1] {
                    nums.swap(j - 1, j);
                    bubble = true;
                }
            }
        } else {
            break;
        }
    }
}
