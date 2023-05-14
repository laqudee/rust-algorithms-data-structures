/// 贪婪方法 —— 找零
/// 有缺陷

pub fn rec_mc1(cashes: &[u32], amount: u32) -> u32 {
    // 全用1元纸币时的最少找零纸币数
    let mut min_cashes = amount;

    if cashes.contains(&amount) {
        return 1;
    } else {
        // 提取符合条件的币种（找零的币值肯定要小于找零值）
        for c in cashes
            .iter()
            .filter(|&c| *c <= amount)
            .collect::<Vec<&u32>>()
        {
            // amount减去c，表示使用了一张面额为c的纸币
            // 所以要加1
            let num_cashes = 1 + rec_mc1(&cashes, amount - c);

            // num_cashes 若比 min_cashes小则更新
            if num_cashes < min_cashes {
                min_cashes = num_cashes;
            }
        }
    }

    min_cashes
}
