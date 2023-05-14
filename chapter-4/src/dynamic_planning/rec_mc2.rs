/// 修复 rec_mc1的缺陷
/// 保存结果

pub fn rec_mc2(cashes: &[u32], amount: u32, min_cashes: &mut [u32]) -> u32 {
    // 全用1元纸币时的最少找零纸币数量
    let mut min_cashes_num = amount;

    if cashes.contains(&amount) {
        return 1;
    } else if min_cashes[amount as usize] > 0 {
        // 找零值 amount 有最小找零纸币数，直接返回
        return min_cashes[amount as usize];
    } else {
        for c in cashes
            .iter()
            .filter(|c| *(*c) <= amount)
            .collect::<Vec<&u32>>()
        {
            let cashe_num = 1 + rec_mc2(&cashes, amount - c, min_cashes);

            // 更新最小找零纸币数量
            if cashe_num < min_cashes_num {
                min_cashes_num = cashe_num;
                min_cashes[amount as usize] = min_cashes_num;
            }
        }
    }

    min_cashes_num
}
