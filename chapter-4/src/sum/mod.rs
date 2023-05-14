pub mod nums_sum;

pub fn it_work() {
    let nums = [2, 1, 7, 4, 5];
    let sum1 = nums_sum::nums_sum1(&nums);
    let sum2 = nums_sum::nums_sum2(&nums);

    println!("sum1: {}, sum2: {}", sum1, sum2);
}
