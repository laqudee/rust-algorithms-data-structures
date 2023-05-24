/// 优化编辑距离，在计算过程中反复利用一个数组来计算和保存值将矩阵缩小为m+1或n+1长度的数组
use std::cmp::min;

pub fn play(source: &str, target: &str) -> usize {
    // 极端情况下：空字符串到字符串的转换
    if source.is_empty() {
        return target.len();
    } else if target.is_empty() {
        return source.len();
    }

    // distance存储了到各种字符串的编辑距离
    let target_c = target.chars().count();
    let mut distances = (0..=target_c).collect::<Vec<_>>();
    for (i, cs) in source.chars().enumerate() {
        let mut substt = i;
        distances[0] = substt + 1;
        for (j, ct) in target.chars().enumerate() {
            let dist = min(
                min(distances[j], distances[j + 1]) + 1,
                substt + (cs != ct) as usize,
            );
            substt = distances[j + 1];
            distances[j + 1] = dist;
        }
    }

    // 最后一个距离值就是最终答案
    distances.pop().unwrap()
}
