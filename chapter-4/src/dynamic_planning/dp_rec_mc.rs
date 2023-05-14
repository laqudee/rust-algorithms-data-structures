/// 动态规划 解决找零问题
/// 减少了栈的使用

pub fn dp_rec_mc(cashes: &[u32], amount: u32, min_cashes: &mut [u32]) -> u32 {
    // 动态收集从1到amount的最小找零币值数量
    // 然后从小到大凑出找零纸币数量
    for denm in 1..=amount {
        let mut min_cashe_num = denm;
        for c in cashes.iter().filter(|&c| *c <= denm).collect::<Vec<&u32>>() {
            let index = (denm - c) as usize;

            let cashes_num = 1 + min_cashes[index];
            if cashes_num < min_cashe_num {
                min_cashe_num = cashes_num;
            }
        }
        min_cashes[denm as usize] = min_cashe_num;
    }

    // 因为收集了 各个值的最小找零纸币数，所以直接返回
    min_cashes[amount as usize]
}
