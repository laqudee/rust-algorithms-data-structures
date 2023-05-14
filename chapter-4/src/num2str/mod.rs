pub mod rec;
pub mod stk;

pub fn it_work() {
    let num = 100;
    let sb = rec::work(num, 2);
    let so = rec::work(num, 8);
    let sh = rec::work(num, 16);
    println!("{num} is b{sb} is o{so} is h{sh}");

    let num = 100;
    let sb = stk::work(num, 2);
    let so = stk::work(num, 8);
    let sh = stk::work(num, 16);
    println!("{num} is b{sb} is o{so} is h{sh}");
}
