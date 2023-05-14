pub mod order_search;
pub mod search_pos;

/// 顺序查找
pub fn search(nums: &[i32], num: i32) -> bool {
    let mut pos = 0;
    let mut found = false;

    // found 表示是否找到
    // pos 在索引范围内且未找到就继续循环
    while pos < nums.len() && !found {
        if num == nums[pos] {
            found = true;
        } else {
            pos += 1;
        }
    }

    found
}

pub fn it_work() {
    let num = 8;
    let nums = [9, 3, 4, 8, 1, 0, 12];
    let found = search(&nums, num);
    println!("{num} is in nums: {}", found);

    let num = 12;
    let found = search_pos::search_pos(&nums, num);
    println!("{num} is in nums: {:?}", found);

    let nums = [
        1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47,
        49, 51, 53, 55, 57, 59, 61, 63, 65, 67, 69, 71, 73, 75, 77, 79, 81, 83, 85, 87, 89, 91, 93,
        95, 97, 99,
    ];
    let num = 31;
    let found = order_search::order_search(&nums, num);
    println!("{num} is in nums: {}", found);
}
