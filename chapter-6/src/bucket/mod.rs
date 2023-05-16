/// 桶排序
use std::fmt::Debug;

// hasher 是一个函数，计算时传入
// values 是数据容器，保存数据
pub struct Bucket<H, T> {
    hasher: H,
    values: Vec<T>,
}

impl<H, T> Bucket<H, T> {
    pub fn new(hasher: H, value: T) -> Bucket<H, T> {
        Bucket {
            hasher,
            values: vec![value],
        }
    }
}

pub fn bucket_sort<H, T, F>(nums: &mut [T], hasher: F)
where
    H: Ord,
    T: Ord + Clone + Debug,
    F: Fn(&T) -> H,
{
    let mut buckets: Vec<Bucket<H, T>> = Vec::new();

    for val in nums.iter() {
        let hasher = hasher(&val);

        // 对桶中数据二分搜索并排序
        match buckets.binary_search_by(|bct| bct.hasher.cmp(&hasher)) {
            Ok(idx) => buckets[idx].values.push(val.clone()),
            Err(idx) => buckets.insert(idx, Bucket::new(hasher, val.clone())),
        }
    }

    // 拆桶，将所有排序数据融合到一个Vec
    let ret = buckets
        .into_iter()
        .flat_map(|mut bucket| {
            bucket.values.sort();
            bucket.values
        })
        .collect::<Vec<T>>();

    nums.clone_from_slice(&ret);
}
pub fn it_work() {
    let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];
    bucket_sort(&mut nums, |t| t / 5);
    println!("bucket sort -- sorted nums: {:?}", nums);
}
