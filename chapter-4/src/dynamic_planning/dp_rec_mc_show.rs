/// 动态规划 解决找零问题
/// 减少了栈的使用
/// 显示所需的纸币面额与数量

pub fn dp_rec_mc_show(
    cashes: &[u32],
    amount: u32,
    min_cashes: &mut [u32],
    cashes_used: &mut [u32],
) -> u32 {
    for denm in 1..=amount {
        let mut min_cashe_num = denm;
        let mut used_cashe = 1; // 最小面额是1元
        for c in cashes.iter().filter(|&c| *c <= denm).collect::<Vec<&u32>>() {
            let index = (denm - c) as usize;

            let cashes_num = 1 + min_cashes[index];
            if cashes_num < min_cashe_num {
                min_cashe_num = cashes_num;
                used_cashe = *c;
            }
        }

        // 更新 各金额对应的最小纸币数
        min_cashes[denm as usize] = min_cashe_num;
        cashes_used[denm as usize] = used_cashe;
    }

    // 因为收集了 各个值的最小找零纸币数，所以直接返回
    min_cashes[amount as usize]
}

pub fn print_cashes(cashes_used: &[u32], mut amount: u32) {
    while amount > 0 {
        let curr = cashes_used[amount as usize];
        println!("${curr}");
        amount -= curr;
    }
}
