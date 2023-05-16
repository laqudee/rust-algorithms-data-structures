/// 归并排序

fn merge_sort(nums: &mut [i32]) {
    if nums.len() > 1 {
        let mid = nums.len() >> 1;
        merge_sort(&mut nums[..mid]); // 排序前半部分
        merge_sort(&mut nums[mid..]); // 排序后半部分
        merge(nums, mid) // 合并排序结果
    }
}

fn merge(nums: &mut [i32], mid: usize) {
    let mut i = 0; // 标记前半部分数据
    let mut k = mid; // 标记后半部分
    let mut temp = Vec::new();

    for _j in 0..nums.len() {
        if k == nums.len() || i == mid {
            break;
        }

        // 数据放到临时集合temp
        if nums[i] < nums[k] {
            temp.push(nums[i]);
            i += 1;
        } else {
            temp.push(nums[k]);
            k += 1;
        }
    }

    // 合并的两部分数据长度大概率不一样长
    // 所以要将为处理完集合的数据全部加入
    if i < mid && k == nums.len() {
        for j in i..mid {
            temp.push(nums[j]);
        }
    } else if i == mid && k < nums.len() {
        for j in k..nums.len() {
            temp.push(nums[j]);
        }
    }

    // temp 数据放回nums，完成排序
    for j in 0..nums.len() {
        nums[j] = temp[j];
    }
}

pub fn it_work() {
    let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];
    merge_sort(&mut nums);
    println!("merge -- sorted nums: {:?}", nums);
}
