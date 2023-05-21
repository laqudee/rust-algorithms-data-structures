/// Rust 实现汉明距离

// 计算数字汉明距离
pub fn hamming_distance1(source: u64, target: u64) -> u32 {
    let mut count = 0;

    let mut xor = source ^ target;

    // 异或取值
    while xor != 0 {
        count += xor & 1;
        xor >>= 1;
    }

    count as u32
}

// Rust 的数字自带一个count_ones()函数用于计算1的个数
pub fn hamming_distance2(source: u64, target: u64) -> u32 {
    (source ^ target).count_ones() as u32
}

// 实现字符版的汉明距离
pub fn hamming_distance_str(source: &str, target: &str) -> u32 {
    let mut count = 0;
    let mut source = source.chars();
    let mut target = target.chars();

    // 两字符串逐字符比较可能出现如下情况
    loop {
        match (source.next(), target.next()) {
            (Some(cs), Some(ct)) if cs != ct => count += 1,
            (Some(_), None) | (None, Some(_)) => panic!("Must have the same length"),
            (None, None) => break,
            _ => continue,
        }
    }

    count as u32
}

pub fn it_work() {
    let source = 1;
    let target = 2;
    let distance = hamming_distance1(source, target);
    println!("the hamming distance is {}", distance);

    let source = 4;
    let target = 6;
    let distance = hamming_distance2(source, target);
    println!("the hamming distance is {}", distance);

    let source = "abce";
    let target = "edcf";
    let distance = hamming_distance_str(source, target);
    println!("the hamming distance is {}", distance);
}
