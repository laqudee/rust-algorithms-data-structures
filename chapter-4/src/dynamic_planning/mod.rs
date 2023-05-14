pub mod rec_mc1;
pub mod rec_mc2;

pub fn it_work() {
    let cashes = [1, 5, 10, 20, 50];
    let amount = 31u32;
    let cashes_num = rec_mc1::rec_mc1(&cashes, amount);
    println!("need refund {}", cashes_num);

    let amount = 81u32;
    let mut min_cashes: [u32; 82] = [0; 82];
    let cashe_num = rec_mc2::rec_mc2(&cashes, amount, &mut min_cashes);
    println!("need refund {} cashes", cashe_num);
}
