/// 递归解决汉诺塔问题

// p: pole 杆
pub fn work(height: u32, src_p: &str, des_p: &str, mid_p: &str) {
    if height >= 1 {
        work(height - 1, src_p, mid_p, des_p);
        println!("moving disk from {src_p} to {des_p}");
        work(height - 1, mid_p, des_p, src_p);
    }
}
