pub fn play(s1: &str, s2: &str) -> bool {
    // 字符串长度不同，一定不是乱序字符串
    if s1.len() != s2.len() {
        return false;
    }

    // s1 和s2中的字符分别加入 alist，blist
    let mut alist = vec![];
    let mut blist = vec![];
    for c in s1.chars() {
        alist.push(c);
    }
    for c in s2.chars() {
        blist.push(c);
    }

    let mut pos1: usize = 0; // pos1,pos2分别表示两个字符串的索引
    let mut ok = true;
    while pos1 < s1.len() && ok {
        let mut pos2: usize = 0;

        // found 标示字符是否在s2中
        let mut found = false;
        while pos2 < blist.len() && !found {
            if alist[pos1] == blist[pos2] {
                found = true;
            } else {
                pos2 += 1;
            }
        }
        // 某字符存在于s2中，将其替换成‘’，避免再次比较
        if found {
            blist[pos2] = ' ';
        } else {
            ok = false;
        }

        // 处理s1中下一个字符
        pos1 += 1;
    }

    ok
}
