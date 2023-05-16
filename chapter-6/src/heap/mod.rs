/// 堆排序

macro_rules! parent {
    // 计算父节点下标宏
    ($child:ident) => {
        $child >> 1
    };
}

macro_rules! left_child {
    // 计算左子节点下标宏
    ($parent:ident) => {
        ($parent << 1) + 1
    };
}

macro_rules! right_child {
    // 计算右子节点下标宏
    ($parent:ident) => {
        ($parent + 1) << 1
    };
}

fn heap_sort(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }

    let len = nums.len();
    let last_parent = parent!(len);
    for i in (0..last_parent).rev() {
        move_down(nums, i); // 第一次建小顶堆
    }

    for end in (1..nums.len()).rev() {
        nums.swap(0, end);
        move_down(&mut nums[..end], 0); // 重建堆
    }
}

fn move_down(nums: &mut [i32], mut parent: usize) {
    let last = nums.len() - 1;
    loop {
        let left = left_child!(parent);
        let right = right_child!(parent);
        if left > last {
            break;
        }

        // right <= last 确保存在右子节点
        let child = if right <= last && nums[left] < nums[right] {
            right
        } else {
            left
        };

        // 子节点大于父节点，交换数据
        if nums[child] > nums[parent] {
            nums.swap(parent, child);
        }

        // 更新父子关系
        parent = child;
    }
}

pub fn it_work() {
    let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];
    heap_sort(&mut nums);
    println!("heap -- sorted nums: {:?}", nums);
}
