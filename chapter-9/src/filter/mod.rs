/// Rust实现各种过滤器
pub mod bloom_filter;

use bloom_filter::BloomFilter;

pub fn it_work() {
    let mut bf = BloomFilter::new(100, 0.08);
    (0..20).for_each(|i| bf.insert(&i));
    let res1 = bf.contains(&2);
    let res2 = bf.contains(&200);
    println!("2 in bf: {res1}, 200 in bf: {res2}");
}
