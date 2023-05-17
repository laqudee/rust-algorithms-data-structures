/// 计数排序

fn counting_sort(nums: &mut [usize]) {
    if nums.len() <= 1 {
        return;
    }

    // 桶数量为nums中最大值加1，保证数据都有桶放
    let max_bkt_num = nums.iter().max().unwrap() + 1;
    let mut counter = vec![0; max_bkt_num];
    for &v in nums.iter() {
        counter[v] += 1; // 将数据标记到桶
    }

    // 数据写回原nums切片
    let mut j = 0;
    for i in 0..max_bkt_num {
        while counter[i] > 0 {
            nums[j] = i;
            counter[i] -= 1;
            j += 1;
        }
    }
}

pub fn it_work() {
    let mut nums = vec![54, 32, 99, 23, 75, 31, 43, 56, 21];
    counting_sort(&mut nums);
    println!("counting -- sorted nums: {:?}", nums);
}
