/// 冒泡排序
pub mod cocktail_sort;

fn bubble_sort1(nums: &mut [i32]) {
    if nums.len() < 2 {
        return;
    }

    for i in 1..nums.len() {
        for j in 0..nums.len() - i {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }
    }
}

fn bubble_sort2(nums: &mut [i32]) {
    let mut len = nums.len() - 1;
    while len > 0 {
        for i in 0..len {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
            }
        }
        len -= 1;
    }
}

fn bubble_sort3(nums: &mut [i32]) {
    let mut compare = true;
    let mut len = nums.len() - 1;

    while len > 0 && compare {
        compare = false;
        for i in 0..len {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
                compare = true; // 数据无序，还需继续比较
            }
        }
        len -= 1;
    }
}

pub fn it_work() {
    let mut nums = [54, 26, 93, 17, 77, 31, 44, 55, 20];
    bubble_sort1(&mut nums);
    println!("sorted nums: {:?}", nums);

    bubble_sort2(&mut nums);
    println!("sorted nums: {:?}", nums);

    bubble_sort3(&mut nums);
    println!("sorted nums: {:?}", nums);

    let mut nums = [1, 3, 2, 8, 3, 6, 4, 9, 5, 10, 6, 7];
    cocktail_sort::sort(&mut nums);
    println!("sorted nums: {:?}", nums);
}
