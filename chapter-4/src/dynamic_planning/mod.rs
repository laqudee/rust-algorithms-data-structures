pub mod dp_rec_mc;
pub mod dp_rec_mc_show;
pub mod rec_mc1;
pub mod rec_mc2;

pub fn it_work() {
    let cashes = [1, 5, 10, 20, 50];
    let amount = 31u32;
    // 有缺陷
    let cashes_num = rec_mc1::rec_mc1(&cashes, amount);
    println!("need refund {}", cashes_num);

    let amount = 81u32;
    let mut min_cashes: [u32; 82] = [0; 82];
    let cashe_num = rec_mc2::rec_mc2(&cashes, amount, &mut min_cashes);
    println!("need refund {} cashes", cashe_num);

    // 动态规划
    let amount = 81u32;
    let cashe_num = dp_rec_mc::dp_rec_mc(&cashes, amount, &mut min_cashes);
    println!("need refund {} cashes", cashe_num);

    // 动态规划 展示纸币面额与数量
    let mut cashes_used: [u32; 82] = [0; 82];
    let amount = 81u32;
    let cs_num = dp_rec_mc_show::dp_rec_mc_show(&cashes, amount, &mut min_cashes, &mut cashes_used);
    println!("need refund {} cashes", cs_num);
    dp_rec_mc_show::print_cashes(&cashes_used, amount)
}
