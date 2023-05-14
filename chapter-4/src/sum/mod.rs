pub mod nums_sum;
pub mod tail_rec;

pub fn it_work() {
    let nums = [2, 1, 7, 4, 5];
    let sum1 = nums_sum::nums_sum1(&nums);
    let sum2 = nums_sum::nums_sum2(&nums);

    println!("sum1: {}, sum2: {}", sum1, sum2);

    let sum3 = tail_rec::nums_sum3(0, &nums);
    let sum4 = tail_rec::nums_sum4(0, &nums);

    println!("sum3: {}, sum4: {}", sum3, sum4);
}
