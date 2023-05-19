/// Rust 实现二叉堆

macro_rules! parent {
    // 计算父节点下标
    ($child:ident) => {
        $child >> 1
    };
}

macro_rules! left_child {
    // 计算左子节点下标
    ($parent:ident) => {
        $parent << 1
    };
}

macro_rules! right_child {
    // 计算右子节点下标
    ($parent:ident) => {
        ($parent << 1) + 1
    };
}

// 二叉堆定义
#[derive(Debug, Clone)]
pub struct BinaryHeap {
    size: usize,    // 数据量
    data: Vec<i32>, // 数据容器
}

impl BinaryHeap {
    pub fn new() -> Self {
        BinaryHeap {
            size: 0, // vec 首位置0，但不计入总数
            data: vec![0],
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn min(&self) -> Option<i32> {
        if self.size == 0 {
            None
        } else {
            // Some(self.data[1].clone()); // 范型数据用clone
            Some(self.data[1])
        }
    }

    // 末尾添加数据，调整堆
    pub fn push(&mut self, val: i32) {
        self.data.push(val);
        self.size += 1;
        self.move_up(self.size);
    }

    // 小数据上冒
    pub fn move_up(&mut self, mut c: usize) {
        loop {
            let p = parent!(c);
            if p <= 0 {
                break;
            }

            if self.data[c] < self.data[p] {
                self.data.swap(c, p);
            }

            c = p;
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        if 0 == self.size {
            // 没数据，返回None
            None
        } else if 1 == self.size {
            self.size -= 1; // 一个数据，比较好处理
            self.data.pop()
        } else {
            // 多个数据，先交换并弹出数据，再调整堆
            self.data.swap(1, self.size);
            let val = self.data.pop();
            self.size -= 1;
            self.move_down(1);
            val
        }
    }

    // 大数据下沉
    pub fn move_down(&mut self, mut c: usize) {
        loop {
            let lc = left_child!(c);
            if lc > self.size {
                break;
            }

            let mc = self.min_child(c);
            if self.data[c] > self.data[mc] {
                self.data.swap(c, mc);
            }
            c = mc;
        }
    }

    // 最小子节点位置
    pub fn min_child(&self, i: usize) -> usize {
        let (lc, rc) = (left_child!(i), right_child!(i));
        if rc > self.size {
            lc
        } else if self.data[lc] < self.data[rc] {
            lc
        } else {
            rc
        }
    }

    // 构建新堆
    pub fn build_new(&mut self, arr: &[i32]) {
        // 删除原始数据
        for _ in 0..self.size {
            let _rm = self.data.pop();
        }

        // 添加新数据
        for &val in arr {
            self.data.push(val);
        }
        self.size = arr.len();

        // 调整小顶堆
        let size = self.size;
        let mut p = parent!(size);
        while p > 0 {
            self.move_down(p);
            p -= 1;
        }
    }

    // 切片数据逐个加入堆
    pub fn build_add(&mut self, arr: &[i32]) {
        for &val in arr {
            self.push(val);
        }
    }
}

pub fn it_work() {
    let mut bh = BinaryHeap::new();
    let nums = [-1, 0, 2, 3, 4];
    bh.push(10);
    bh.push(9);
    bh.push(8);
    bh.push(7);
    bh.push(6);

    bh.build_add(&nums);
    println!("empty: {:?}", bh.is_empty());
    println!("min: {:?}", bh.min());
    println!("pop: {:?}", bh.pop());

    bh.build_new(&nums);
    println!("size: {:?}", bh.len());
    println!("pop min: {:?}", bh.pop());
}
